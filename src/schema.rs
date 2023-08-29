// @generated automatically by Diesel CLI.

diesel::table! {
    bills (id) {
        id -> Int4,
        provider -> Nullable<Varchar>,
        btype -> Nullable<Varchar>,
        amount -> Nullable<Float8>,
        ogbody -> Nullable<Text>,
        date -> Nullable<Varchar>,
        fullpaid -> Nullable<Bool>,
        santipaid -> Nullable<Bool>,
        johnpaid -> Nullable<Bool>,
        liampaid -> Nullable<Bool>,
        sampaid -> Nullable<Bool>,
    }
}

diesel::table! {
    guests (id) {
        id -> Int4,
        gname -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        msg -> Text,
        coming -> Bool,
    }
}

diesel::table! {
    poems (id) {
        id -> Int4,
        title -> Nullable<Text>,
        poem -> Nullable<Text>,
        poet -> Nullable<Text>,
        tags -> Nullable<Text>,
    }
}

diesel::table! {
    sophie_guests (id) {
        id -> Int4,
        gname -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        msg -> Text,
        coming -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bills,
    guests,
    poems,
    sophie_guests,
);
