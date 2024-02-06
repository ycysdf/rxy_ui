
/*

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug, Reflect, Serialize, Deserialize)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct OptionalOverflow {
    pub x: Option<OverflowAxis>,
    pub y: Option<OverflowAxis>,
}

#[derive(Debug, PartialEq, Default, Clone, Copy, Reflect)]
#[reflect(Default, PartialEq)]
pub struct OptionalTransform {
    pub translation: Option<Vec3>,
    pub rotation: Option<Quat>,
    pub scale: Option<Vec3>,
}

impl OptionalTransform {
    pub fn is_some(&self) -> [bool; 3] {
        [
            self.translation.is_some(),
            self.rotation.is_some(),
            self.scale.is_some(),
        ]
    }
}

#[derive(Copy, Clone, Default, PartialEq, Debug, Reflect)]
#[reflect(PartialEq)]
pub struct UiOptionalRect {
    pub left: Option<Val>,
    pub right: Option<Val>,
    pub top: Option<Val>,
    pub bottom: Option<Val>,
}

impl UiOptionalRect {
    pub fn all(val: Val) -> Self {
        Self {
            left: Some(val),
            right: Some(val),
            top: Some(val),
            bottom: Some(val),
        }
    }
    pub fn values(&self) -> [&Option<Val>; 4] {
        [&self.left, &self.right, &self.top, &self.bottom]
    }
    pub fn zero() -> Self {
        Self {
            left: Some(Val::Px(0.)),
            right: Some(Val::Px(0.)),
            top: Some(Val::Px(0.)),
            bottom: Some(Val::Px(0.)),
        }
    }

    pub const fn new(left: Val, right: Val, top: Val, bottom: Val) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
            top: Some(top),
            bottom: Some(bottom),
        }
    }

    pub const fn px(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left: Some(Val::Px(left)),
            right: Some(Val::Px(right)),
            top: Some(Val::Px(top)),
            bottom: Some(Val::Px(bottom)),
        }
    }

    pub const fn percent(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left: Some(Val::Percent(left)),
            right: Some(Val::Percent(right)),
            top: Some(Val::Percent(top)),
            bottom: Some(Val::Percent(bottom)),
        }
    }

    pub fn horizontal(value: Val) -> Self {
        Self {
            left: Some(value),
            right: Some(value),
            ..Default::default()
        }
    }

    pub fn vertical(value: Val) -> Self {
        Self {
            top: Some(value),
            bottom: Some(value),
            ..Default::default()
        }
    }

    pub fn axes(horizontal: Val, vertical: Val) -> Self {
        Self {
            left: Some(horizontal),
            right: Some(horizontal),
            top: Some(vertical),
            bottom: Some(vertical),
        }
    }

    pub fn left(value: Val) -> Self {
        Self {
            left: Some(value),
            ..Default::default()
        }
    }

    pub fn right(value: Val) -> Self {
        Self {
            right: Some(value),
            ..Default::default()
        }
    }

    pub fn top(value: Val) -> Self {
        Self {
            top: Some(value),
            ..Default::default()
        }
    }

    pub fn bottom(value: Val) -> Self {
        Self {
            bottom: Some(value),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Reflect, Clone)]
pub struct TextSections(pub Vec<TextSection>);

impl PartialEq for TextSections {
    fn eq(&self, other: &Self) -> bool {
        self.reflect_partial_eq(other).unwrap_or(false)
    }
}

impl From<String> for TextSections {
    fn from(value: String) -> Self {
        Self(vec![TextSection::new(value, TextStyle::default())])
    }
}

impl<'a> From<&'a str> for TextSections {
    fn from(value: &'a str) -> Self {
        Self(vec![TextSection::new(value, TextStyle::default())])
    }
}



impl_default_attr_values! {
    TextSections,
    UiOptionalRect,
    OptionalOverflow,
    OptionalTransform
}
*/