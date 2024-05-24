use crate::{ImageLinks, CHARS};
use md5::{Digest, Md5};
use regex::Regex;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
extern crate ureq;
//use ureq::Error;
//use std::fs::OpenOptions;

#[derive(Deserialize, Debug)]
struct Imageresponse {
    #[serde(rename = "cargoquery")]
    cargoquery: Vec<Imagedata>,
}

#[derive(Deserialize, Debug)]
struct Imagedata {
    #[serde(rename = "title")]
    title: Imagetitle,
}

#[derive(Deserialize, Debug)]
struct Imagetitle {
    input: Option<String>,
    name: Option<String>,
    images: Option<String>,
    hitboxes: Option<String>,
}

const IMAGE_HALF: &str = "https://www.dustloop.com/wiki/images";

pub async fn images_to_json(
    mut char_page_response_json: String,
    mut file: &File,
    char_count: usize,
) {
    char_page_response_json = char_page_response_json.replace(r#""c."#, r#""近"#);
    char_page_response_json = char_page_response_json.replace(r#""f."#, r#""遠"#);
    char_page_response_json = char_page_response_json.replace(r#""j."#, r#""j"#);
    char_page_response_json = char_page_response_json.replace(r#"Ultimate"#, "U");
    char_page_response_json = char_page_response_json.replace(r#"Ulitmate"#, "U");
    // char_page_response_json = char_page_response_json.replace(r#"~"#, "");

    // println!("{}", char_page_response_json);
    // 2B
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"R030: Hammer""#,
        r#""R030:ハンマー(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"R030: Hammer""#,
        r#""R030:ハンマー(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"A160: Missile""#,
        r#""A160:ミサイル(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"A090: Wire""#,
        r#""A090:ワイヤー(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"R020: Mirage""#,
        r#""R020:ミラージュ(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"A150: Volt""#,
        r#""A150:ヴォルト(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"A080: Wave""#,
        r#""A080:ウェーブ(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"A140: Gravity""#,
        r#""A140:グラビティ(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"A130: Bomb""#,
        r#""A130:ボム(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"Pod C""#,
        r#""ポッドC(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"Pod A""#,
        r#""ポッドA(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"Pod B""#,
        r#""ポッドB(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"R010: Laser""#,
        r#""R010:レーザー(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Android Kick Sequence""#,
        r#""機蹴連携[強化装レーザー](236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Extermination Blade""#,
        r#""殲滅斬機[強化装スピアー](236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""720U","name":"Self-Destruct""#,
        r#""自爆(720U)","name":"720U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Evasive Maneuver""#,
        r#""回避行動(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5UU","name":"Counterstrike""#,
        r#""転進重撃(5UU)","name":"5UU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""6U","name":"Evasive Maneuver""#,
        r#""回避行動(6U)","name":"6U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""jU","name":"Grab Pod""#,
        r#""ポッド掴まり(jU)","name":"jU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""jUU","name":"Grab Pod Follow-up""#,
        r#""ポッド掴まり追加攻撃(jUU)","name":"jUU""#,
    );
    // アニラ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Flock to the Future""#,
        r#""H前途羊羊(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214H4X","name":"H And Beyond!""#,
        r#""H意気羊羊(214H4X)","name":"214H4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Flock to the Future""#,
        r#""L前途羊羊(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L4X","name":"L And Beyond!""#,
        r#""L意気羊羊(214L4X)","name":"214L4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Flock to the Future""#,
        r#""M前途羊羊(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M4X","name":"M And Beyond!""#,
        r#""M意気羊羊(214M4X)","name":"214M4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Fluff 'Em, Stuff 'Em\!""#,
        r#""U前途羊羊(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Over the Fence""#,
        r#""H飛羊乗雲(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Over the Fence""#,
        r#""L飛羊乗雲(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Over the Fence""#,
        r#""M飛羊乗雲(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Over the Fence""#,
        r#""U飛羊乗雲(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Fluff 'Em, Stuff 'Em!""#,
        r#""H駆けよ、もふもふ！(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Fluff 'Em, Stuff 'Em!""#,
        r#""L駆けよ、もふもふ！(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Fluff 'Em, Stuff 'Em!""#,
        r#""M駆けよ、もふもふ！(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Flock to the Future""#,
        r#""U駆けよ、もふもふ！(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H All is Vanity""#,
        r#""H色即是空(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L All is Vanity""#,
        r#""L色即是空(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M All is Vanity""#,
        r#""M色即是空(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U All is Vanity""#,
        r#""U色即是空(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Gilded Heaven Strike""#,
        r#""金牙神然(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Fury of the Ram""#,
        r#""羊爛怒涛(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Ramification""#,
        r#""牙羊乱舞(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5UU","name":"Ramification Follow-up""#,
        r#""牙羊乱舞（追加攻撃）(5UU)","name":"5UU""#,
    );
    // ウーノ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Mantra Wheel""#,
        r#""H摩尼車(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Mantra Wheel""#,
        r#""L摩尼車(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Mantra Wheel""#,
        r#""M摩尼車(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Mantra Wheel""#,
        r#""U摩尼車(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Rakshasa""#,
        r#""H羅刹槍(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Rakshasa""#,
        r#""L羅刹槍(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Rakshasa""#,
        r#""M羅刹槍(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Rakshasa""#,
        r#""U羅刹槍(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X-X","name":"Rakshasa Followup""#,
        r#""真撃(236X-X)","name":"236X-X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Spiral Spear""#,
        r#""H螺旋回鉾(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Spiral Spear""#,
        r#""L螺旋回鉾(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Spiral Spear""#,
        r#""M螺旋回鉾(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Spiral Spear""#,
        r#""U螺旋回鉾(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623X-6H","name":"Peerless Thrust""#,
        r#""無双閃(623X-6H)","name":"623X-6H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623X-6L","name":"Radiance Ruination""#,
        r#""輝極衝(623X-6L)","name":"623X-6L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623X-6M","name":"Astral Piercer""#,
        r#""星断(623X-6M)","name":"623X-6M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623X-6U","name":"Grand Haste""#,
        r#""速疾(623X-6U)","name":"623X-6U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214H","name":"Air H Mantra Wheel""#,
        r#""空中H摩尼車(j214H)","name":"j214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214L","name":"Air L Mantra Wheel""#,
        r#""空中L摩尼車(j214L)","name":"j214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214M","name":"Air M Mantra Wheel""#,
        r#""空中M摩尼車(j214M)","name":"j214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214U","name":"U Air Mantra Wheel""#,
        r#""空中U摩尼車(j214U)","name":"j214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8H","name":"H Fleeting Spark""#,
        r#""H刹那の閃き([2]8H)","name":"[2]8H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8L","name":"L Fleeting Spark""#,
        r#""L刹那の閃き([2]8L)","name":"[2]8L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8M","name":"M Fleeting Spark""#,
        r#""M刹那の閃き([2]8M)","name":"[2]8M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8U","name":"U Fleeting Spark""#,
        r#""U刹那の閃き([2]8U)","name":"[2]8U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214214H","name":"Seven Spears of Lightning""#,
        r#""雷光七本槍(214214H)","name":"214214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Astralance: Everto""#,
        r#""天逆鉾(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Thousand-Spear Void""#,
        r#""千槍無量曼荼羅(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Arm the Bastion""#,
        r#""城廓の構え(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j2U","name":"One Rift Spear""#,
        r#""豪槍(j2U)","name":"j2U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j6U","name":"One Rift Spear""#,
        r#""豪槍(j6U)","name":"j6U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""jU","name":"Arm the Bastion""#,
        r#""城廓の構え(jU)","name":"jU""#,
    );
    // アバタール・ベリアル
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Carnal Passion""#,
        r#""H煮エ滾ル熱情(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Carnal Passion""#,
        r#""M煮エ滾ル熱情(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Carnal Passion""#,
        r#""U煮エ滾ル熱情(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214X4X","name":"Followup 1""#,
        r#""煮エ滾ル熱情追加攻撃(214X4X)","name":"214X4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214X4X4X","name":"Followup 2""#,
        r#""煮エ滾ル熱情追加攻撃2(214X4X4X)","name":"214X4X4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Wings Bestowed""#,
        r#""H賜リシ翼(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H~H","name":"H Barbatos""#,
        r#""Hバルバトス(22HH)","name":"22HH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Wings Bestowed""#,
        r#""L賜リシ翼(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L~H","name":"L Barbatos""#,
        r#""Lバルバトス(22LH)","name":"22LH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Wings Bestowed""#,
        r#""M賜リシ翼(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M~H","name":"M Barbatos""#,
        r#""Mバルバトス(22MH)","name":"22MH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Wings Bestowed""#,
        r#""U賜リシ翼(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~L","name":"Stolas""#,
        r#""ストラス(22X~L)","name":"22X~L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~M","name":"Beleth""#,
        r#""ベレト(22X~M)","name":"22X~M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~U","name":"Habakkuk (Midair)""#,
        r#""ハバクク(22X~U)","name":"22X~U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~[U]","name":"Habakkuk (Midair, Charged)""#,
        r#""ハバクク（タメ）(22X~[U])","name":"22X~[U]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Rejected Truth""#,
        r#""H否定サレタ真実(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"Rejected Truth""#,
        r#""L否定サレタ真実(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"Destructive Delusions""#,
        r#""M嘘ノ上塗リ(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Destructive Delusions""#,
        r#""U嘘ノ上塗リ(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Blind Devotion""#,
        r#""H盲目的ナ信心(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Blind Devotion""#,
        r#""L盲目的ナ信心(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Blind Devotion""#,
        r#""M盲目的ナ信心(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Blind Devotion""#,
        r#""U盲目的ナ信心(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Diafthora""#,
        r#""ディアプトラ(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Wild Sin""#,
        r#""ワイルド・シン(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j236236H","name":"Diafthora""#,
        r#""空中ディアプトラ(j236236H)","name":"j236236H""#,
    );
    char_page_response_json = char_page_response_json
        .replace(r#""5U","name":"Habakkuk""#, r#""ハバクク(5U)","name":"5U""#);
    char_page_response_json = char_page_response_json.replace(
        r#""5U Follow-Up","name":"Pact (5U)""#,
        r#""聖約(5U Follow-Up)","name":"5U Follow-Up""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5[U]","name":"Habakkuk (Charged)""#,
        r#""ハバクク（タメ）(5[U])","name":"5[U]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""8U Follow-Up","name":"Pact (8U)""#,
        r#""パクト（上）(8U Follow-Up)","name":"8U Follow-Up""#,
    );

    // ベアトリクス
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Thunderbird""#,
        r#""Hサンダーバード(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Thunderbird""#,
        r#""Lサンダーバード(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Thunderbird""#,
        r#""Mサンダーバード(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Thunderbird""#,
        r#""Uサンダーバード(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Riding Free""#,
        r#""Hライディングフリー(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Riding Free""#,
        r#""Lライディングフリー(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Riding Free""#,
        r#""Mライディングフリー(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Riding Free""#,
        r#""Uライディングフリー(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~X","name":"Sapphire Slice""#,
        r#""ライディングフリー追撃(22XX)","name":"22XX""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Star Chaser""#,
        r#""Hスターチェイサー(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Star Chaser""#,
        r#""Lスターチェイサー(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Star Chaser""#,
        r#""Mスターチェイサー(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Star Chaser""#,
        r#""Uスターチェイサー(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Embrasque Sword""#,
        r#""Hエムブラスクの剣(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Embrasque Sword""#,
        r#""Lエムブラスクの剣(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Embrasque Sword""#,
        r#""Mエムブラスクの剣(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Embrasque Sword""#,
        r#""Uエムブラスクの剣(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Immortal Assault""#,
        r#""イモータル・アソールト(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Savior Unleashed""#,
        r#""セイヴィアー・アンリーシュド(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Nothing Is Forever""#,
        r#""ナッシング・イズ・フォエヴァー(5U)","name":"5U""#,
    );

    // ベリアル
    let mut re = Regex::new(r#""input":"(214)([LMHU])","name":"[LMHU] Asmodeus""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2アスモデウス($1$2)","name":"$1$2""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(236)([LMHU])","name":"[LMHU] Goetia""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2ゴエティア($1$2)","name":"$1$2""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(236X)~(4)([LMH])","name":"[LMH] Everyone's A Little Crooked""#)
        .unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$3少々屈曲するのは普通だろう？($1$2$3)","name":"$1$2$3""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(623)([LMHU])","name":"[LMHU] Notoria""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2ノトリア($1$2)","name":"$1$2""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(632146)([LMHU])","name":"[LMHU] Vassago""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2ヴァサーゴ($1$2)","name":"$1$2""#,
        )
        .to_string();

    char_page_response_json = char_page_response_json.replace(
        r#""632146U Catch","name":"U Vassago Catch""#,
        r#""ヴァサーゴ（当身）(632146U Catch)","name":"632146U Catch""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""632146U Throw","name":"U Vassago Throw""#,
        r#""ヴァサーゴ（投げ）(632146U Throw)","name":"632146U Throw""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Legemeton""#,
        r#""レメゲトン(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Anagenesis""#,
        r#""アナゲンネーシス(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Give Daddy Some Sugar""#,
        r#""無価値なもの(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U Catch","name":"Give Daddy Some Sugar (Attack)""#,
        r#""無価値なもの（当身）(5U Catch)","name":"5U Catch""#,
    );

    // カリオストロ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Alexandria""#,
        r#""Hアレクサンドリア(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Alexandria""#,
        r#""Lアレクサンドリア(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Alexandria""#,
        r#""Mアレクサンドリア(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Alexandria""#,
        r#""Uアレクサンドリア(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Spare Body""#,
        r#""Hスペアボディ(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Spare Body""#,
        r#""Lスペアボディ(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Spare Body""#,
        r#""Mスペアボディ(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Spare Body""#,
        r#""Uスペアボディ(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Mehen""#,
        r#""Hメヘン(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Mehen""#,
        r#""Lメヘン(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Mehen""#,
        r#""Mメヘン(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Mehen""#,
        r#""Uメヘン(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Calculated""#,
        r#""Hスペキュレーション(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Calculated""#,
        r#""Lスペキュレーション(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Calculated""#,
        r#""Mスペキュレーション(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Calculated""#,
        r#""Uスペキュレーション(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22H","name":"Air H Spare Body""#,
        r#""空中Hスペアボディ(j22H)","name":"j22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22L","name":"Air L Spare Body""#,
        r#""空中Lスペアボディ(j22L)","name":"j22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22M","name":"Air M Spare Body""#,
        r#""空中Mスペアボディ(j22M)","name":"j22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22U","name":"U Spare Body (Air)""#,
        r#""空中Uスペアボディ(j22U)","name":"j22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j236H","name":"Air H Mehen""#,
        r#""空中Hメヘン(j236H)","name":"j236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j236L","name":"Air L Mehen""#,
        r#""空中Lメヘン(j236L)","name":"j236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j236M","name":"Air M Mehen""#,
        r#""空中Mメヘン(j236M)","name":"j236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j236U","name":"U Mehen (Air)""#,
        r#""空中Uメヘン(j236U)","name":"j236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Ars Magna""#,
        r#""アルス・マグナ(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Everything's Coming Up Cagliostro""#,
        r#""あなたもわたしもカリオストロ☆(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Collapse (Conjure)""#,
        r#""コラプス（生成）(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""6U Level 1","name":"Collapse (Fire)""#,
        r#""コラプス（発射）Lv1(6U Level 1)","name":"6U Level 1""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""6U Level 2","name":"Collapse (Fire)""#,
        r#""コラプス（発射）Lv2(6U Level 2)","name":"6U Level 2""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""6U Level 3","name":"Collapse (Fire)""#,
        r#""コラプス（発射）Lv3(6U Level 3)","name":"6U Level 3""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""6U Level 4","name":"Collapse (Fire)""#,
        r#""コラプス（発射）Lv4(6U Level 4)","name":"6U Level 4""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""6U Level 5","name":"Collapse (Fire)""#,
        r#""コラプス（発射）Lv5(6U Level 5)","name":"6U Level 5""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j2/6U Level 1","name":"Collapse (Fire)""#,
        r#""空中コラプス（発射 Lv1）Lv1(j2/6U Level 1)","name":"j2/6U Level 1""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j2/6U Level 2","name":"Collapse (Fire)""#,
        r#""空中コラプス（発射）Lv2(j2/6U Level 2)","name":"j2/6U Level 2""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j2/6U Level 3","name":"Collapse (Fire)""#,
        r#""空中コラプス（発射）Lv3(j2/6U Level 3)","name":"j2/6U Level 3""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j2/6U Level 4","name":"Collapse (Fire)""#,
        r#""空中コラプス（発射）Lv4(j2/6U Level 4)","name":"j2/6U Level 4""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j2/6U Level 5","name":"Collapse (Fire)""#,
        r#""空中コラプス（発射）Lv5(j2/6U Level 5)","name":"j2/6U Level 5""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""jU","name":"Collapse (Conjure)""#,
        r#""空中コラプス（生成）(jU)","name":"jU""#,
    );

        // シャルロッテ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Sword of Lumiel""#,
        r#""Hソード・オブ・リュミエール(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Sword of Lumiel""#,
        r#""Lソード・オブ・リュミエール(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Sword of Lumiel""#,
        r#""Mソード・オブ・リュミエール(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Sword of Lumiel""#,
        r#""Uソード・オブ・リュミエール(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Noble Strategy""#,
        r#""Uノーブルストラテジー(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X","name":"Noble Strategy""#,
        r#""ノーブルストラテジー(22X)","name":"22X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~5","name":"With Surest Strike!""#,
        r#""素早く隙を突くであります(22X5)","name":"22X5""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~H","name":"With Sweetest Skills!""#,
        r#""これがキマったらカッコいいであります(22XH)","name":"22XH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~L","name":"With Flying Nobility!""#,
        r#""遙か上空から急襲するであります(22XL)","name":"22XL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~M","name":"With Purest Bravery!""#,
        r#""勇ましく斬るあります(22XM)","name":"22XM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8H","name":"H Holy Ladder""#,
        r#""Hホーリーラダー([2]8H)","name":"[2]8H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8L","name":"L Holy Ladder""#,
        r#""Lホーリーラダー([2]8L)","name":"[2]8L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8M","name":"M Holy Ladder""#,
        r#""Mホーリーラダー([2]8M)","name":"[2]8M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8U","name":"U Holy Ladder""#,
        r#""Uホーリーラダー([2]8U)","name":"[2]8U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6H","name":"H Shining Onslaught""#,
        r#""Hグリッターオンスロート([4]6H)","name":"[4]6H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6L","name":"L Shining Onslaught""#,
        r#""Lグリッターオンスロート([4]6L)","name":"[4]6L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6M","name":"M Shining Onslaught""#,
        r#""Mグリッターオンスロート([4]6M)","name":"[4]6M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6U","name":"U Shining Onslaught""#,
        r#""Uグリッターオンスロート([4]6U)","name":"[4]6U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Brilliant Moon""#,
        r#""ブリリアントムーン(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Noble Execution""#,
        r#""ノーブル・エクスキューション(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Königsschild""#,
        r#""ケーニシルト(5U)","name":"5U""#,
    );
    // let mut re = Regex::new(r"c\.").unwrap();
    // char_page_response_json = re.replace_all(&char_page_response_json, "近").to_string();
    // re = Regex::new(r"f\.").unwrap();
    // char_page_response_json = re.replace_all(&char_page_response_json, "遠").to_string();

    /*
    let mut re = Regex::new(r#""(214)([LMHU])""#).unwrap();
    char_page_response_json = re
        .replace_all(&char_page_response_json, r#""$2ヤクザキック($1$2)""#)
        .to_string();

    let mut re = Regex::new(r#""(236)([LMHU])""#).unwrap();
    char_page_response_json = re
        .replace_all(&char_page_response_json, r#""$2レギンレイヴ($1$2)""#)
        .to_string();

    let mut re = Regex::new(r#""(623)([LMHU])""#).unwrap();
    char_page_response_json = re
        .replace_all(&char_page_response_json, r#""$2ライジングソード($1$2)""#)
        .to_string();
    */

    let mut imagedata: Imageresponse = serde_json::from_str(&char_page_response_json).unwrap();

    for x in 0..imagedata.cargoquery.len() {
        // Variable that the produced hitbox links will reside
        let mut hitboxes_link: Vec<String> = Vec::new();
        // Variable that the produced image link will reside
        let image_link;

        // Replacing None values with a generic '-'
        if imagedata.cargoquery[x].title.input.is_none() {
            imagedata.cargoquery[x].title.input = Some("".to_string());
        } else {
            // Skips finish blow for sol
            if *imagedata.cargoquery[x].title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
        }
        if imagedata.cargoquery[x].title.name.is_none() {
            imagedata.cargoquery[x].title.name = Some(
                imagedata.cargoquery[x]
                    .title
                    .input
                    .as_ref()
                    .unwrap()
                    .to_string(),
            );
        } else {
            // Skips dash cancel entry ino hoverdash chipp escape zato flight and finish blow
            if imagedata.cargoquery[x]
                .title
                .name
                .as_ref()
                .unwrap()
                .to_string()
                .trim()
                == "Dash Cancel"
                || imagedata.cargoquery[x]
                    .title
                    .name
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .trim()
                    == "Hoverdash"
                || imagedata.cargoquery[x]
                    .title
                    .name
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .trim()
                    == "Finish Blow"
                || imagedata.cargoquery[x]
                    .title
                    .name
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .trim()
                    == "Flight"
                || imagedata.cargoquery[x]
                    .title
                    .name
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .trim()
                    == "Escape"
            {
                continue;
            }
        }
        if imagedata.cargoquery[x].title.images.is_none() {
            image_link = "".to_string();
        } else {
            // If image field contains only spaces
            if imagedata.cargoquery[x]
                .title
                .images
                .as_ref()
                .unwrap()
                .trim()
                == ""
            {
                image_link = "".to_string();
            } else {
                // Multiple image names
                // Removing any subsequent image names from field
                if imagedata.cargoquery[x]
                    .title
                    .images
                    .as_mut()
                    .unwrap()
                    .contains('\\')
                {
                    let split_image: Vec<&str> = imagedata.cargoquery[x]
                        .title
                        .images
                        .as_mut()
                        .unwrap()
                        .split('\\')
                        .collect();

                    imagedata.cargoquery[x].title.images =
                        Some(split_image[0].to_string().replace(' ', "_"));

                    // Sending image name to make_link to become a link
                    image_link = make_link(
                        imagedata.cargoquery[x]
                            .title
                            .images
                            .as_ref()
                            .unwrap()
                            .to_string(),
                    )
                    .await;
                } else {
                    // Single image name
                    imagedata.cargoquery[x].title.images = Some(
                        imagedata.cargoquery[x]
                            .title
                            .images
                            .as_ref()
                            .unwrap()
                            .to_string()
                            .replace(' ', "_"),
                    );
                    // Sending image name to make_link to become a link
                    image_link = make_link(
                        imagedata.cargoquery[x]
                            .title
                            .images
                            .as_ref()
                            .unwrap()
                            .to_string(),
                    )
                    .await;
                }
            }
        }

        // If hitbox empty
        if imagedata.cargoquery[x].title.hitboxes.is_none() {
            hitboxes_link.push("".to_string());
        } else {
            // // If image field contains only spaces
            // if imagedata.cargoquery[x].title.hitboxes.as_ref().unwrap().trim() == "" {
            //     hitboxes_link.push("".to_string());
            // }
            // Remove any hitbox images for throws cause they dont exist
            if imagedata.cargoquery[x]
                .title
                .hitboxes
                .as_ref()
                .unwrap()
                .trim()
                .to_lowercase()
                .contains("6d")
            {
                hitboxes_link.push("".to_string());
            } else {
                // Splitting the hitboxes names into a vector
                let hitbox_str: Vec<&str> = imagedata.cargoquery[x]
                    .title
                    .hitboxes
                    .as_ref()
                    .unwrap()
                    .split('\\')
                    .collect();

                for hitbox_string in &hitbox_str {
                    // Sending hitbox names to make_link to become a vector of links
                    hitboxes_link
                        .push(make_link(hitbox_string.to_string().trim().replace(' ', "_")).await);
                }
            }
        }

        // Serializing image data
        let processed_imagedata = serde_json::to_string_pretty(&ImageLinks {
            input: imagedata.cargoquery[x]
                .title
                .input
                .as_ref()
                .unwrap()
                .to_string(),
            move_img: image_link,
            hitbox_img: hitboxes_link,
        })
        .unwrap();

        write!(file, "{}", processed_imagedata)
            .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count] + ".json'."));

        // Skip writting comma/tab if next and last iteration
        // contains 'finish blow' in last the input field
        if x == imagedata.cargoquery.len() - 2
            && *imagedata.cargoquery[x + 1].title.input.as_ref().unwrap()
                == "j.XX during Homing Jump"
        {
            continue;
        } else if x != imagedata.cargoquery.len() - 1 {
            // Adding comma/tab
            //file.write(b",\n\t")
            //    .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS[char_count]+ ".json'."));
            (&mut file).write_all(b",\n\t").expect(
                &("\nFailed to write ',\\n\\t' while serializing '".to_owned()
                    + CHARS[char_count]
                    + ".json'."),
            );
        }
    }
}

async fn make_link(image_name: String) -> String {
    let image_bytes = image_name.as_bytes();

    // Creating a Md5 hasher instance
    let mut hasher = Md5::new();
    hasher.update(image_bytes);
    // Converting hex to string
    let result = format!("{:x}", hasher.finalize());
    // Getting the first two hex digits from the md5sum
    // let char1 = result.chars().nth(0).unwrap();
    let char1 = result.chars().next().unwrap();
    let char2 = result.chars().nth(1).unwrap();
    // Making final link by concating
    // https://www.dustloop.com/wiki/images/first hex digit/first hex second hex/image names with underscores instead of spaces
    let image_link = format!("{}/{}/{}{}/{}", IMAGE_HALF, char1, char1, char2, image_name);

    // // Debug testing links and outputting the broken ones in a file
    // match ureq::get(&image_link).call() {
    //     Ok(_) => {},
    //     Err(Error::Status(code, _/*response*/)) => {
    //         // Creating character images json file
    //         let mut file = OpenOptions::new()
    //             .create(true)
    //             .append(true)
    //             .open("broken_links.txt")
    //             .expect("\nFailed to open 'broken_links.txt'");

    //         write!(file, "Code: {}, Link: {}\n", code, image_link)
    //             .expect("\nFailed to write to 'broken_links.txt'");
    //     }
    //     Err(_) => {}
    // }

    image_link
}
