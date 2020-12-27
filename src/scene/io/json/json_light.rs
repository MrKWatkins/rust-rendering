use crate::scene::io::json::{JsonColour, JsonPoint, JsonScalar};
use crate::scene::light::LightSampling;
use crate::scene::Light;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonLight {
    pub position: JsonPoint,
    pub colour: JsonColour,

    #[serde(flatten)]
    #[serde(rename = "type")]
    pub light_type: JsonLightType,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JsonLightType {
    Point,
    Spherical {
        radius: JsonScalar,

        #[serde(default = "default_sampling")]
        sampling: JsonLightSamplingType,

        #[serde(default = "default_samples")]
        samples: usize,

        cache_samples: Option<bool>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JsonLightSamplingType {
    Random,
    Uniform,
}

fn default_sampling() -> JsonLightSamplingType {
    return JsonLightSamplingType::Random;
}

fn default_samples() -> usize {
    return 10;
}

fn default_cache_samples(light_type: &JsonLightSamplingType) -> bool {
    return match light_type {
        JsonLightSamplingType::Random => false,
        JsonLightSamplingType::Uniform => true,
    };
}

impl JsonLight {
    pub fn to_light(&self) -> Light {
        let position = self.position.to_point();
        let colour = self.colour.to_colour();

        return match &self.light_type {
            JsonLightType::Point => Light::point(position, colour),
            JsonLightType::Spherical {
                radius,
                sampling,
                samples,
                cache_samples,
            } => {
                let cache = cache_samples.unwrap_or(default_cache_samples(sampling));

                return Light::spherical(position, colour, *radius, sampling.to_light_sampling(), *samples, cache);
            }
        };
    }
}

impl JsonLightSamplingType {
    pub fn to_light_sampling(&self) -> LightSampling {
        return match self {
            JsonLightSamplingType::Random => LightSampling::Random,
            JsonLightSamplingType::Uniform => LightSampling::Uniform,
        };
    }
}
