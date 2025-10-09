/// TODO doc
#[macro_export]
macro_rules! animation_set {
    ($name:ident [ $($content:tt)* ]) => {
        animation_set!(@ $name {} {} $($content)*);
    };

    // Optional clip
    (@ $name:ident {$($fields:tt)*} {$($current:tt)*} clip $field:ident ? $(, $($rest:tt)*)?) => {
        animation_set!(@ $name {$($fields)* $field: Option<ClipId>,} {} $($($rest)*)?);
    };

    // Required clip
    (@ $name:ident {$($fields:tt)*} {$($current:tt)*} clip $field:ident $(, $($rest:tt)*)?) => {
        animation_set!(@ $name {$($fields)* $field: ClipId,} {} $($($rest)*)?);
    };

    // Optional animation
    (@ $name:ident {$($fields:tt)*} {$($current:tt)*} anim $field:ident ? $(, $($rest:tt)*)?) => {
        animation_set!(@ $name {$($fields)* $field: Option<Handle<Animation>>,} {} $($($rest)*)?);
    };

    // Required animation
    (@ $name:ident {$($fields:tt)*} {$($current:tt)*} anim $field:ident $(, $($rest:tt)*)?) => {
        animation_set!(@ $name {$($fields)* $field: Handle<Animation>,} {} $($($rest)*)?);
    };

    // Optional marker
    (@ $name:ident {$($fields:tt)*} {$($current:tt)*} marker $field:ident ? $(, $($rest:tt)*)?) => {
        animation_set!(@ $name {$($fields)* $field: Option<MarkerId>,} {} $($($rest)*)?);
    };

    // Required marker
    (@ $name:ident {$($fields:tt)*} {$($current:tt)*} marker $field:ident $(, $($rest:tt)*)?) => {
        animation_set!(@ $name {$($fields)* $field: MarkerId,} {} $($($rest)*)?);
    };

    // Struct
    (@ $name:ident {$($fields:tt)*} {}) => {
        #[derive(Resource)]
        struct $name {
            $($fields)*
        }
    };
}
