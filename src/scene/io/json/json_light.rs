use crate::scene::io::json::{JsonColour, JsonPoint, JsonScalar};
use crate::scene::light::LightSampling;
use crate::scene::{Attenuation, Light};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonLight {
    pub position: JsonPoint,
    pub colour: JsonColour,
    pub attenuation: Option<JsonAttenuation>,
    #[serde(flatten)]
    pub light_type: Option<JsonLightType>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum JsonAttenuation {
    None,
    Linear {
        half_intensity_distance: JsonScalar,
    },
    Inverse {
        half_intensity_distance: JsonScalar,
    },
    #[serde(rename = "inverse squared")]
    InverseSquared {
        half_intensity_distance: JsonScalar,
    },
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
        let attenuation = self.attenuation.as_ref().map_or(Attenuation::None, |a| a.to_attenuation());
        let light_type = self.light_type.as_ref().unwrap_or(&JsonLightType::Point);

        return match light_type {
            JsonLightType::Point => Light::point(position, colour, attenuation),
            JsonLightType::Spherical {
                radius,
                sampling,
                samples,
                cache_samples,
            } => {
                let cache = cache_samples.unwrap_or(default_cache_samples(&sampling));

                return Light::spherical(position, colour, *radius, attenuation, sampling.to_light_sampling(), *samples, cache);
            }
        };
    }
}

impl JsonAttenuation {
    pub fn to_attenuation(&self) -> Attenuation {
        return match self {
            JsonAttenuation::None => Attenuation::new_none(),
            JsonAttenuation::Linear { half_intensity_distance } => Attenuation::new_linear(*half_intensity_distance),
            JsonAttenuation::Inverse { half_intensity_distance } => Attenuation::new_inverse(*half_intensity_distance),
            JsonAttenuation::InverseSquared { half_intensity_distance } => Attenuation::new_inverse_squared(*half_intensity_distance),
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
