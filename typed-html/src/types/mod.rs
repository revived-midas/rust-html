mod class;
pub use self::class::Class;

mod id;
pub use self::id::Id;

mod spacedlist;
pub use self::spacedlist::SpacedList;

mod spacedset;
pub use self::spacedset::SpacedSet;

pub type ClassList = SpacedSet<Class>;

pub use http::Uri;
pub use language_tags::LanguageTag;
pub use mime::Mime;

pub type Target = String;
pub type CharacterEncoding = String;
pub type FeaturePolicy = String;

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum ButtonType {
        #[strum(to_string = "submit")]
        Submit,
        #[strum(to_string = "reset")]
        Reset,
        #[strum(to_string = "button")]
        Button,
    }
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum CrossOrigin {
        #[strum(to_string = "anonymous")]
        Anonymous,
        #[strum(to_string = "use-credentials")]
        UseCredentials,
    }
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum FormEncodingType {
        #[strum(to_string = "application/x-www-form-urlencoded")]
        UrlEncoded,
        #[strum(to_string = "multipart/form-data")]
        FormData,
        #[strum(to_string = "text/plain")]
        Text,
    }
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum FormMethod {
        #[strum(to_string = "post")]
        Post,
        #[strum(to_string = "get")]
        Get,
    }
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum ImageDecoding {
        #[strum(to_string = "sync")]
        Sync,
        #[strum(to_string = "async")]
        Async,
        #[strum(to_string = "auto")]
        Auto,
    }
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum InputType {
        #[strum(to_string = "button")]
        Button,
        #[strum(to_string = "checkbox")]
        Checkbox,
        #[strum(to_string = "color")]
        Color,
        #[strum(to_string = "date")]
        Date,
        #[strum(to_string = "datetime-local")]
        DatetimeLocal,
        #[strum(to_string = "email")]
        Email,
        #[strum(to_string = "file")]
        File,
        #[strum(to_string = "hidden")]
        Hidden,
        #[strum(to_string = "image")]
        Image,
        #[strum(to_string = "month")]
        Month,
        #[strum(to_string = "number")]
        Number,
        #[strum(to_string = "password")]
        Password,
        #[strum(to_string = "radio")]
        Radio,
        #[strum(to_string = "range")]
        Range,
        #[strum(to_string = "reset")]
        Reset,
        #[strum(to_string = "search")]
        Search,
        #[strum(to_string = "submit")]
        Submit,
        #[strum(to_string = "tel")]
        Tel,
        #[strum(to_string = "text")]
        Text,
        #[strum(to_string = "time")]
        Time,
        #[strum(to_string = "url")]
        Url,
        #[strum(to_string = "week")]
        Week,
    }
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum LinkType {
        #[strum(to_string = "alternate")]
        Alternate,
        #[strum(to_string = "author")]
        Author,
        #[strum(to_string = "bookmark")]
        Bookmark,
        #[strum(to_string = "canonical")]
        Canonical,
        #[strum(to_string = "external")]
        External,
        #[strum(to_string = "help")]
        Help,
        #[strum(to_string = "icon")]
        Icon,
        #[strum(to_string = "license")]
        License,
        #[strum(to_string = "manifest")]
        Manifest,
        #[strum(to_string = "modulepreload")]
        ModulePreload,
        #[strum(to_string = "next")]
        Next,
        #[strum(to_string = "nofollow")]
        NoFollow,
        #[strum(to_string = "noopener")]
        NoOpener,
        #[strum(to_string = "noreferrer")]
        NoReferrer,
        #[strum(to_string = "pingback")]
        PingBack,
        #[strum(to_string = "prefetch")]
        Prefetch,
        #[strum(to_string = "preload")]
        Preload,
        #[strum(to_string = "prev")]
        Prev,
        #[strum(to_string = "search")]
        Search,
        #[strum(to_string = "shortlink")]
        ShortLink,
        #[strum(to_string = "stylesheet")]
        StyleSheet,
        #[strum(to_string = "tag")]
        Tag,
    }
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum OnOff {
        #[strum(to_string = "on")]
        On,
        #[strum(to_string = "off")]
        Off,
    }
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum Preload {
        #[strum(to_string = "none")]
        None,
        #[strum(to_string = "metadata")]
        Metadata,
        #[strum(to_string = "auto")]
        Auto,
    }
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum ReferrerPolicy {
        #[strum(to_string = "no-referrer")]
        NoReferrer,
        #[strum(to_string = "no-referrer-when-downgrade")]
        NoReferrerWhenDowngrade,
        #[strum(to_string = "origin")]
        Origin,
        #[strum(to_string = "origin-when-cross-origin")]
        OriginWhenCrossOrigin,
        #[strum(to_string = "unsafe-url")]
        UnsafeUrl,
    }
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum Sandbox {
        #[strum(to_string = "allow-forms")]
        AllowForms,
        #[strum(to_string = "allow-modals")]
        AllowModals,
        #[strum(to_string = "allow-orientation-lock")]
        AllowOrientationLock,
        #[strum(to_string = "allow-pointer-lock")]
        AllowPointerLock,
        #[strum(to_string = "allow-popups")]
        AllowPopups,
        #[strum(to_string = "allow-popups-to-escape-sandbox")]
        AllowPopupsToEscapeSandbox,
        #[strum(to_string = "allow-presentation")]
        AllowPresentation,
        #[strum(to_string = "allow-same-origin")]
        AllowSameOrigin,
        #[strum(to_string = "allow-scripts")]
        AllowScripts,
        #[strum(to_string = "allow-top-navigation")]
        AllowTopNavigation,
        #[strum(to_string = "allow-top-navigation-by-user-navigation")]
        AllowTopNavigationByUserNavigation,
    }
}

enum_set_type! {
    #[derive(EnumString, Display)]
    pub enum TextDirection {
        #[strum(to_string = "ltr")]
        LeftToRight,
        #[strum(to_string = "rtl")]
        RightToLeft,
    }
}
