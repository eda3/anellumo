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
    char_page_response_json = char_page_response_json.replace(r#" &gt; "#, "");

    char_page_response_json = char_page_response_json.replace(
        r#""BC","name":"Brave Counter""#,
        r#""ブレイブカウンター","name":"ブレイブカウンター""#,
    );
    char_page_response_json = char_page_response_json.replace(r#"L+U"#, r#"LU"#);
    char_page_response_json = char_page_response_json.replace(
        r#""jLU","name":"Air Throw""#,
        r#""空投げ(jLU)","name":"jLU""#,
    );
    char_page_response_json = char_page_response_json
        .replace(r#""LU","name":"Ground Throw""#, r#""投げ(LU)","name":"LU""#);
    char_page_response_json = char_page_response_json.replace(
        r#""MH","name":"Raging Strike""#,
        r#""レイジングストライク(MH)","name":"MH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""MH~MH","name":"Raging Chain""#,
        r#""レイジングチェイン(MHMH)","name":"MHMH""#,
    );

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

    // ユーステス
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Close Combat""#,
        r#""Hクロース・コンバット(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214HH","name":"Assault Knife""#,
        r#""アサルトナイフ(214HH)","name":"214HH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214HL","name":"Stunlight""#,
        r#""スタンライト(214HL)","name":"214HL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214HM","name":"Takedown""#,
        r#""テイクダウン(214HM)","name":"214HM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214HU","name":"Spreadshot""#,
        r#""スプレッドショット(214HU)","name":"214HU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Close Combat""#,
        r#""Lクロース・コンバット(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214LH","name":"Assault Knife""#,
        r#""アサルトナイフ(214LH)","name":"214LH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214LL","name":"Stunlight""#,
        r#""スタンライト(214LL)","name":"214LL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214LM","name":"Takedown""#,
        r#""テイクダウン(214LM)","name":"214LM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214LU","name":"Spreadshot""#,
        r#""スプレッドショット(214LU)","name":"214LU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Close Combat""#,
        r#""Mクロース・コンバット(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214MH","name":"Assault Knife""#,
        r#""アサルトナイフ(214MH)","name":"214MH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214ML","name":"Stunlight""#,
        r#""スタンライト(214ML)","name":"214ML""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214MM","name":"Takedown""#,
        r#""テイクダウン(214MM)","name":"214MM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214MU","name":"Spreadshot""#,
        r#""スプレッドショット(214MU)","name":"214MU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Assault Knife""#,
        r#""Uクロース・コンバット(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214XG","name":"Brake""#,
        r#""ブレーキ(214XG)","name":"214XG""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Rat Race""#,
        r#""Hラット・レース(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Rat Race""#,
        r#""Lラット・レース(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Rat Race""#,
        r#""Mラット・レース(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Rat Race""#,
        r#""Uラット・レース(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8H","name":"H Slow Kill""#,
        r#""Hスロー・キル([2]8H)","name":"[2]8H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8L","name":"L Slow Kill""#,
        r#""Lスロー・キル([2]8L)","name":"[2]8L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8M","name":"M Slow Kill""#,
        r#""Mスロー・キル([2]8M)","name":"[2]8M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8U","name":"U Slow Kill""#,
        r#""Uスロー・キル([2]8U)","name":"[2]8U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8[H]","name":"H Slow Kill (Charged)""#,
        r#""Hスロー・キル（タメ）([2]8[H])","name":"[2]8[H]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8[L]","name":"L Slow Kill (Charged)""#,
        r#""Lスロー・キル（タメ）([2]8[L])","name":"[2]8[L]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8[M]","name":"M Slow Kill (Charged)""#,
        r#""Mスロー・キル（タメ）([2]8[M])","name":"[2]8[M]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6H","name":"H Flamek Thunder""#,
        r#""Hフラメクの雷([4]6H)","name":"[4]6H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6L","name":"L Flamek Thunder""#,
        r#""Lフラメクの雷([4]6L)","name":"[4]6L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6M","name":"M Flamek Thunder""#,
        r#""Mフラメクの雷([4]6M)","name":"[4]6M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6U","name":"U Flamek Thunder""#,
        r#""Uフラメクの雷([4]6U)","name":"[4]6U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6[H]","name":"H Flamek Thunder (Charged)""#,
        r#""Hフラメクの雷（タメ）([4]6[H])","name":"[4]6[H]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6[L]","name":"L Flamek Thunder (Charged)""#,
        r#""Lフラメクの雷（タメ）([4]6[L])","name":"[4]6[L]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6[M]","name":"M Flamek Thunder (Charged)""#,
        r#""Mフラメクの雷（タメ）([4]6[M])","name":"[4]6[M]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Acidrage Howl""#,
        r#""アシッドレイジ・ロアー(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Dead-End Fall""#,
        r#""デッドエンド・フォール(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Stay Tonight""#,
        r#""ステイ・トゥナイト(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5UU","name":"Followup""#,
        r#""ステイ・トゥナイト追加攻撃(5UU)","name":"5UU""#,
    );

    // フェリ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Trombe""#,
        r#""Hトロンペ(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Trombe""#,
        r#""Lトロンペ(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Trombe""#,
        r#""Mトロンペ(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Trombe""#,
        r#""Uトロンペ(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Geegee, Get 'Em!""#,
        r#""H足止めだ、ジジ！(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Geegee, Get 'Em!""#,
        r#""L足止めだ、ジジ！(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Geegee, Get 'Em!""#,
        r#""M足止めだ、ジジ！(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Geegee, Get 'Em!""#,
        r#""U足止めだ、ジジ！(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Gespenst""#,
        r#""Hゲシュペンスト(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Gespenst""#,
        r#""Lゲシュペンスト(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Gespenst""#,
        r#""Mゲシュペンスト(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Gespenst""#,
        r#""Uゲシュペンスト(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X4X","name":"Heel""#,
        r#""こっちに来い(236X4X)","name":"236X4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236XX","name":"Whip It Good""#,
        r#""お仕置きだ(236XX)","name":"236XX""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Beppo, Sic 'Em!""#,
        r#""H迎え撃て、ベッポ！(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Beppo, Sic 'Em!""#,
        r#""L迎え撃て、ベッポ！(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Beppo, Sic 'Em!""#,
        r#""M迎え撃て、ベッポ！(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Beppo, Sic 'Em!""#,
        r#""U迎え撃て、ベッポ！(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214214H","name":"Hinrichten""#,
        r#""ヒン・リヒテン(214214H)","name":"214214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Vergiften""#,
        r#""フェアギフテン(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Aetheryte Requiescat""#,
        r#""エーテライト・レクイエスカ(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Ein Ball""#,
        r#""アイン・バル(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j4U","name":"Spectral Dive""#,
        r#""ドローレン(j4U)","name":"j4U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j5U/6U","name":"Spectral Dive""#,
        r#""ドローレン(j5U/6U)","name":"j5U/6U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j7U","name":"Ghostwing""#,
        r#""空中移動(j7U)","name":"j7U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j8U","name":"Ghostwing""#,
        r#""空中移動(j8U)","name":"j8U""#,
    );

    // グリームニル
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Deathrite Tempest""#,
        r#""H葬嵐神滅穿(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Deathrite Tempest""#,
        r#""L葬嵐神滅穿(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Deathrite Tempest""#,
        r#""M葬嵐神滅穿(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"Deathrite Temptest""#,
        r#""U葬嵐神滅穿(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Wind Crest""#,
        r#""Hウィンドエングライヴ(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Wind Crest""#,
        r#""Lウィンドエングライヴ(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Wind Crest""#,
        r#""Mウィンドエングライヴ(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"Wind Crest""#,
        r#""Uウィンドエングライヴ(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Dancing Storm""#,
        r#""Hダンシングストーム(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H4X","name":"H Wind Shift""#,
        r#""軌道変化左(236H4X)","name":"236H4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H6X","name":"H Wind Shift""#,
        r#""軌道変化右(236H6X)","name":"236H6X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H8X","name":"H Wind Shift""#,
        r#""軌道変化上(236H8X)","name":"236H8X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Dancing Storm""#,
        r#""Lダンシングストーム(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Dancing Storm""#,
        r#""Mダンシングストーム(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X4X","name":"L/M Wind Shift""#,
        r#""L/M軌道変化(236X~4X)","name":"236X4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X6X","name":"L/M Wind Shift""#,
        r#""L/M軌道変化(236X~6X)","name":"236X6X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X8X","name":"L/M Wind Shift""#,
        r#""L/M軌道変化(236X~8X)","name":"236X8X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Winds of Temptation""#,
        r#""Hギルティサイクロン(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Winds of Temptation""#,
        r#""Lギルティサイクロン(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Winds of Temptation""#,
        r#""Mギルティサイクロン(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"Winds of Temptation""#,
        r#""Uギルティサイクロン(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214H","name":"H Voidlight Erasure""#,
        r#""H虚幻晄裂葬(j214H)","name":"j214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214L","name":"L Voidlight Erasure""#,
        r#""L虚幻晄裂葬(j214L)","name":"j214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214M","name":"M Voidlight Erasure""#,
        r#""M虚幻晄裂葬(j214M)","name":"j214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214U","name":"Voidlight Erasure""#,
        r#""U虚幻晄裂葬(j214U)","name":"j214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214214H","name":"Grace of the Radiant""#,
        r#""天佑神助晄(214214H)","name":"214214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Holy Ray of Purification""#,
        r#""神聖破邪晄(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Terminal Vortex""#,
        r#""エンドテンペスト(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"Dancing Storm""#,
        r#""ダンシングストーム(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json
        .replace(r#""5U","name":"Galewing""#, r#""天翔疾駆(5U)","name":"5U""#);
    char_page_response_json = char_page_response_json.replace(
        r#""XU","name":"Gale Dash""#,
        r#""ゲイルダッシュ(XU)","name":"XU""#,
    );

    // カタリナ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Enchanted Lands""#,
        r#""Hエンチャントランズ(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Enchanted Lands""#,
        r#""Lエンチャントランズ(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Enchanted Lands""#,
        r#""Mエンチャントランズ(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Enchanted Lands""#,
        r#""Uエンチャントランズ(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Frozen Blade""#,
        r#""Hマイディライド(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Frozen Blade""#,
        r#""Lマイディライド(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Frozen Blade""#,
        r#""Mマイディライド(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Frozen Blade""#,
        r#""Uマイディライド(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Emerald Sword""#,
        r#""Hエメラルドソード(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Emerald Sword""#,
        r#""Lエメラルドソード(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Emerald Sword""#,
        r#""Mエメラルドソード(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Emerald Sword""#,
        r#""Uエメラルドソード(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Blades of Frost""#,
        r#""アイシクルネイル(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Realm of Ice""#,
        r#""ヴィジョン・ディヴァイン(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U44","name":"Backstep""#,
        r#""バックステップ(5U44)","name":"5U44""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U66","name":"Frontstep""#,
        r#""フロントステップ(5U66)","name":"5U66""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U Lv0","name":"Light Wall""#,
        r#""ライトウォールLv0(5U Lv0)","name":"5U Lv0""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U Lv1","name":"Light Wall""#,
        r#""ライトウォールLv1(5U Lv1)","name":"5U Lv1""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U Lv2","name":"Light Wall""#,
        r#""ライトウォールLv2(5U Lv2)","name":"5U Lv2""#,
    );

    // ファスティバ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Elegant Lariat""#,
        r#""H漢女のラリアット(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Elegant Lariat""#,
        r#""L漢女のラリアット(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Elegant Lariat""#,
        r#""M漢女のラリアット(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Elegant Lariat""#,
        r#""U漢女のラリアット(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Headbutt of Love (1)""#,
        r#""H愛のヘッドバット（1）(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H6H","name":"H Headbutt of Love (2)""#,
        r#""H愛のヘッドバット（2）(236H6H)","name":"236H6H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H6HH","name":"H Headbutt of Love (3)""#,
        r#""H愛のヘッドバット（3）(236H6HH)","name":"236H6HH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Headbutt of Love""#,
        r#""L愛のヘッドバット(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Headbutt of Love""#,
        r#""M愛のヘッドバット(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Headbutt of Love""#,
        r#""U愛のヘッドバット(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""360H","name":"H Jewel Resort Screwdriver""#,
        r#""Hスクリュー・ジュエルバスター(360H)","name":"360H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""360L","name":"L Jewel Resort Screwdriver""#,
        r#""Lスクリュー・ジュエルバスター(360L)","name":"360L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""360M","name":"M Jewel Resort Screwdriver""#,
        r#""Mスクリュー・ジュエルバスター(360M)","name":"360M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""360U","name":"U Jewel Resort Screwdriver""#,
        r#""Uスクリュー・ジュエルバスター(360U)","name":"360U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Devoted Body Slam""#,
        r#""H真心のボディ・スラム(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Devoted Body Slam""#,
        r#""L真心のボディ・スラム(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Devoted Body Slam""#,
        r#""M真心のボディ・スラム(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Devoted Body Slam""#,
        r#""U真心のボディ・スラム(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j360H","name":"H Leg Drop of Adoration""#,
        r#""H慈しみのレッグ・ドロップ(j360H)","name":"j360H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j360L","name":"L Leg Drop of Adoration""#,
        r#""L慈しみのレッグ・ドロップ(j360L)","name":"j360L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j360M","name":"M Leg Drop of Adoration""#,
        r#""M慈しみのレッグ・ドロップ(j360M)","name":"j360M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j360U","name":"U Leg Drop of Adoration""#,
        r#""U慈しみのレッグ・ドロップ(j360U)","name":"j360U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214214H","name":"Maximum Love Bomb""#,
        r#""ラヴマックス・ボンバー(214214H)","name":"214214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Maximum Love Bomb""#,
        r#""ラヴマックス・ボンバー(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""720U","name":"The Shape of Love""#,
        r#""ある愛のカタチ～ジュエル・リゾート編～(720U)","name":"720U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Love Grapple""#,
        r#""抱きしめてア・ゲ・ル！(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5UH","name":"Hurl""#,
        r#""ドラゴンホイップH(5UH)","name":"5UH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5UL","name":"Hurl""#,
        r#""ドラゴンホイップ(5UL)","name":"5UL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5UM","name":"Hurl""#,
        r#""ドラゴンホイップM(5UM)","name":"5UM""#,
    );

    // ランスロット
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Southern Cross""#,
        r#""Hサザンクロス(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214H~4H","name":"H Southern Cross (2,3,4)""#,
        r#""Hサザンクロス(214H4H)","name":"214H4H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Southern Cross""#,
        r#""Lサザンクロス(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Southern Cross""#,
        r#""Mサザンクロス(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Southern Cross""#,
        r#""Uサザンクロス(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214X~4X","name":"Southern Cross (2)""#,
        r#""サザンクロス追加攻撃(214X4X)","name":"214X4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214X~4X Ender","name":"Southern Cross Ender""#,
        r#""サザンクロスエンダー(214X4X Ender)","name":"214X4X Ender""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Turbulenz""#,
        r#""Hトゥルブレンツ(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Turbulenz""#,
        r#""Lトゥルブレンツ(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Turbulenz""#,
        r#""Mトゥルブレンツ(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Turbulenz""#,
        r#""Uトゥルブレンツ(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22[X]","name":"Turbulenz (Charged)""#,
        r#""トゥルブレンツ（タメ）(22[X])","name":"22[X]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Wogenstrom""#,
        r#""Hヴォーゲンシュトローム(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Wogenstrom""#,
        r#""Lヴォーゲンシュトローム(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Wogenstrom""#,
        r#""Mヴォーゲンシュトローム(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Wogenstrom""#,
        r#""Uヴォーゲンシュトローム(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Blade Impulse""#,
        r#""Hブレードインパルス(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Blade Impulse""#,
        r#""Lブレードインパルス(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Blade Impulse""#,
        r#""Mブレードインパルス(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Blade Impulse""#,
        r#""Uブレードインパルス(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Weiss Flugel""#,
        r#""ヴァイス・フィリューゲル(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Schöner Winterhügel""#,
        r#""シューナー・ヴィンターヒューゲル(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j236236U","name":"Schöner Winterhügel""#,
        r#""空中シューナー・ヴィンターヒューゲル(j236236U)","name":"j236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Wirbelwind""#,
        r#""ヴィルベルヴィント(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U~2U","name":"Jump""#,
        r#""ジャンプ(5U2U)","name":"5U2U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U~4U","name":"Feint""#,
        r#""フェイント(5U4U)","name":"5U4U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U~5U","name":"Quick Stop""#,
        r#""急停止(5U5U)","name":"5U5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U~6U","name":"Cross Over""#,
        r#""回り込み(5U6U)","name":"5U6U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""jU","name":"Wirbelwind""#,
        r#""空中ヴィルベルヴィント(jU)","name":"jU""#,
    );

    // ローアイン
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Awesome Sauce""#,
        r#""Hポコパン(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Awesome Sauce""#,
        r#""Lポコパン(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Awesome Sauce""#,
        r#""Mポコパン(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Awesome Sauce""#,
        r#""Uポコパン(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Magnificent Tool of Destruction""#,
        r#""H戦争兵器(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Magnificent Tool of Destruction""#,
        r#""L戦争兵器(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Magnificent Tool of Destruction""#,
        r#""M戦争兵器(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Magnificent Tool of Destruction""#,
        r#""U戦争兵器(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Sammy &amp; Tommy""#,
        r#""Hエルっち&トモちゃん(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Sammy &amp; Tommy""#,
        r#""Lエルっち&トモちゃん(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Sammy &amp; Tommy""#,
        r#""Mエルっち&トモちゃん(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Sammy &amp; Tommy""#,
        r#""Uエルっち&トモちゃん(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Come at Me, Bro!""#,
        r#""Hエルっち&トモちゃん(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Come at Me, Bro!""#,
        r#""LAventure(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Come at Me, Bro!""#,
        r#""MAventure(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Come at Me, Bro!""#,
        r#""UAventure(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""l.214U","name":"U Awesome Sauce (Lucky)""#,
        r#""Uポコパン（ラッキー）(l214U)","name":"l214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Human! Pyramid! Attack!""#,
        r#""超一騎当千(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H 236U","name":"Moment of Truth""#,
        r#""勝負所 ※KBSN解除されます（笑）(236236H236U)","name":"236236H236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H G","name":"Totally Rad Juke""#,
        r#""半端ない回避(236236HG)","name":"236236HG""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H H","name":"Flex On 'Em""#,
        r#""漢斬り(236236HH)","name":"236236HH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H L","name":"It's Lit, Bros""#,
        r#""オラついた攻撃(236236HL)","name":"236236HL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H M","name":"Definitely Don't Try This at Home""#,
        r#""絶対に真似しないでください(236236HM)","name":"236236HM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H U","name":"Catch 'Em Slippin'""#,
        r#""膝スラ(236236HU)","name":"236236H U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Try This on for Size!""#,
        r#""サイズ感的に勝ち目ねーから！(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U H","name":"Nether Veil""#,
        r#""ネザーマントル(236236UH)","name":"236236U H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U L","name":"Earth Pillar""#,
        r#""ランドピラー(236236UL)","name":"236236UL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U M","name":"Axis Mundi""#,
        r#""アクシスムンディ(236236UM)","name":"236236UM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U U","name":"Luminox Genesi""#,
        r#""創世のルミノックス(236236UU)","name":"236236UU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Don't Mind If I Do""#,
        r#""シーメーマシマシ(5U)","name":"5U""#,
    );

    // ルシファー
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Iscariot""#,
        r#""Hイスカリオテ(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Iscariot""#,
        r#""Lイスカリオテ(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Iscariot""#,
        r#""Mイスカリオテ(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Iscariot""#,
        r#""Uイスカリオテ(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Tessera""#,
        r#""Hテセラ(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Tessera""#,
        r#""Lテセラ(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Tessera""#,
        r#""Mテセラ(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Tessera""#,
        r#""Uテセラ(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Iblis""#,
        r#""Hイブリース(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Iblis""#,
        r#""Lイブリース(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Iblis""#,
        r#""Mイブリース(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Iblis""#,
        r#""Uイブリース(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Phosphorus""#,
        r#""Hポースポロス(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Phosphorus""#,
        r#""Lポースポロス(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Phosphorus""#,
        r#""Mポースポロス(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Phosphorus""#,
        r#""Uポースポロス(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214214H","name":"Axion Control""#,
        r#""アキシオン・コントロール(214214H)","name":"214214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Axion""#,
        r#""アキシオン(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Paradise Lost""#,
        r#""パラダイス・ロスト(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Orbital Resonance""#,
        r#""オービタル・レゾナンス(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U~H","name":"Orbital Resonance ~ H""#,
        r#""Hオービタル・レゾナンス追加攻撃(5U~H)","name":"5U~H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U~L","name":"Orbital Resonance ~ L""#,
        r#""Lオービタル・レゾナンス追加攻撃(5UL)","name":"5UL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U~M","name":"Orbital Resonance ~ M""#,
        r#""Mオービタル・レゾナンス追加攻撃(5UM)","name":"5UM""#,
    );

    // メーテラ
    // メーテラ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Starry Sky (Low)""#,
        r#""H綺羅星（下段）(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Starry Sky (Low)""#,
        r#""L綺羅星（下段）(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Starry Sky (Low)""#,
        r#""M綺羅星（下段）(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Starry Sky (Low)""#,
        r#""U綺羅星（下段）(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Aetherial Seal""#,
        r#""Hエーテリアルシール(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Aetherial Seal""#,
        r#""Lエーテリアルシール(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Aetherial Seal""#,
        r#""Mエーテリアルシール(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Aetherial Seal""#,
        r#""Uエーテリアルシール(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Starry Sky (High)""#,
        r#""H綺羅星（上段）(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Starry Sky (High)""#,
        r#""L綺羅星（上段）(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Starry Sky (High)""#,
        r#""M綺羅星（上段）(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Starry Sky (High)""#,
        r#""U綺羅星（上段）(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H The Great Fall""#,
        r#""H隼落とし(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L The Great Fall""#,
        r#""L隼落とし(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M The Great Fall""#,
        r#""M隼落とし(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U The Great Fall""#,
        r#""U隼落とし(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214H","name":"Air H Starry Sky (Low)""#,
        r#""H空中綺羅星（上段）(j214H)","name":"j214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214L","name":"Air L Starry Sky (Low)""#,
        r#""L空中綺羅星（上段）(j214L)","name":"j214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214M","name":"Air M Starry Sky (Low)""#,
        r#""M空中綺羅星（上段）(j214M)","name":"j214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214U","name":"U Aerial Starry Sky (Low)""#,
        r#""U空中綺羅星（上段）(j214U)","name":"j214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j236H","name":"Air H Starry Sky (High)""#,
        r#""H空中綺羅星（下段）(j236H)","name":"j236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j236L","name":"Air L Starry Sky (High)""#,
        r#""L空中綺羅星（下段）(j236L)","name":"j236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j236M","name":"Air M Starry Sky (High)""#,
        r#""M空中綺羅星（下段）(j236M)","name":"j236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j236U","name":"U Aerial Starry Sky (High)""#,
        r#""U空中綺羅星（下段）(j236U)","name":"j236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Dense Caress""#,
        r#""デンス・キャレス(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Rapid Pulverize""#,
        r#""ラピッド・パルパーション(236236U)","name":"236236U""#,
    );
    char_page_response_json =
        char_page_response_json.replace(r#""5U","name":"Zephyr""#, r#""ゼファー(5U)","name":"5U""#);
    char_page_response_json = char_page_response_json.replace(
        r#""jU","name":"Zephyr""#,
        r#""空中ゼファー(jU)","name":"jU""#,
    );

    // ナルメア
    char_page_response_json = char_page_response_json.replace(
        r#""214H[g]","name":"H Transient""#,
        r#""H泡沫(214Hg)","name":"214Hg""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L[g]","name":"L Transient""#,
        r#""L泡沫(214Lg)","name":"214Lg""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M[g]","name":"M Transient""#,
        r#""M泡沫(214Mg)","name":"214Mg""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U[g]","name":"U Transient""#,
        r#""U泡沫(214Ug)","name":"214Ug""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H[g]","name":"H Setsuna""#,
        r#""H刹那(236Hg)","name":"236Hg""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L[g]","name":"L Setsuna""#,
        r#""L刹那(236Lg)","name":"236Lg""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M[g]","name":"M Setsuna""#,
        r#""M刹那(236Mg)","name":"236Mg""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U[g]","name":"U Setsuna""#,
        r#""U刹那(236Ug)","name":"236Ug""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H[g]","name":"H Absolute Horizon""#,
        r#""H万里一空(623Hg)","name":"623Hg""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L[g]","name":"L Absolute Horizon""#,
        r#""L万里一空(623Lg)","name":"623Lg""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M[g]","name":"M Absolute Horizon""#,
        r#""M万里一空(623Mg)","name":"623Mg""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U[g]","name":"U Absolute Horizon""#,
        r#""U万里一空(623Ug)","name":"623Ug""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214H[k]","name":"H Crescent Moon""#,
        r#""H繊月(214Hk)","name":"214Hk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L[k]","name":"L Crescent Moon""#,
        r#""L繊月(214Lk)","name":"214Lk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M[k]","name":"M Crescent Moon""#,
        r#""M繊月(214Mk)","name":"214Mk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U[k]","name":"U Crescent Moon""#,
        r#""U繊月(214Uk)","name":"214Uk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H[k]","name":"H Kyokasuigetsu""#,
        r#""H鏡花水月(236Hk)","name":"236Hk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L[k]","name":"L Kyokasuigetsu""#,
        r#""L鏡花水月(236Lk)","name":"236Lk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M[k]","name":"M Kyokasuigetsu""#,
        r#""M鏡花水月(236Mk)","name":"236Mk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U[k]","name":"U Kyokasuigetsu""#,
        r#""U鏡花水月(236Uk)","name":"236Uk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H[k]","name":"H Mugenkagura""#,
        r#""H夢幻神楽(623Hk)","name":"623Hk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L[k]","name":"L Mugenkagura""#,
        r#""L夢幻神楽(623Lk)","name":"623Lk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M[k]","name":"M Mugenkagura""#,
        r#""M夢幻神楽(623Mk)","name":"623Mk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U[k]","name":"U Mugenkagura""#,
        r#""U夢幻神楽(623Uk)","name":"623Uk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214H[k]","name":"H Crescent Moon (Midair)""#,
        r#""空中H繊月(j214Hk)","name":"j214Hk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214L[k]","name":"L Crescent Moon (Midair)""#,
        r#""空中L繊月(j214Lk)","name":"j214Lk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214M[k]","name":"M Crescent Moon (Midair)""#,
        r#""空中M繊月(j214Mk)","name":"j214Mk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214U[k]","name":"U Aerial Crescent Moon""#,
        r#""空中U繊月(j214Uk)","name":"j214Uk""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"One Hundred Cloudscapes""#,
        r#""紫雲百景(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Butterfly Effect: Ame-no-Uzume""#,
        r#""胡蝶刃・天鈿女命舞(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Butterfly Effect""#,
        r#""胡蝶刃(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(r#"[g]"#, r#"g"#);
    char_page_response_json = char_page_response_json.replace(r#"[k]"#, r#"k"#);

    // ニーア
    re = Regex::new(r#""input":"(214)([LMHU])","name":"[LMHU] Ominous Turn""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2オミノスターン($1$2)","name":"$1$2""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(236)([LMHU])","name":"[LMHU] Fatal Waltz""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2死の舞踏($1$2)","name":"$1$2""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(623)([LMHU])","name":"[LMHU] Misfortune""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2ミスフォーチュン($1$2)","name":"$1$2""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(22)([LMHU])","name":"[LMHU] Death, Reversed""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2リヴァース($1$2)","name":"$1$2""#,
        )
        .to_string();

    char_page_response_json = char_page_response_json.replace(
        r#""214214H","name":"World of Death and Love""#,
        r#""死ト愛ノ世界(214214H)","name":"214214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Klagen Totentanz""#,
        r#""クラーゲン・トーテンタンツ(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Drowned in an Exquisite and Eternal Adieu""#,
        r#""熱情ノ花、芳醇タル別レニ溺ルル(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Love's Redemption""#,
        r#""愛ノ救イ(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5[U]","name":"Love's Redemption (Charged)""#,
        r#""愛ノ救イ（タメ）(5[U])","name":"5[U]""#,
    );
    char_page_response_json = char_page_response_json.replace(r#" [d]"#, r#"d"#);

    // パーシヴァル
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Lord's Strike""#,
        r#""H王者の行進(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214HH","name":"H Zerreissen""#,
        r#""Hツェアライセン(214HH)","name":"214HH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214HL","name":"H Schneiden""#,
        r#""Hシュナイデン(214HL)","name":"214HL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214HM","name":"H Macht""#,
        r#""Hマハト(214HM)","name":"214HM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Lord's Strike""#,
        r#""L王者の行進(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214LH","name":"L Zerreissen""#,
        r#""Lツェアライセン(214LH)","name":"214LH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214LL","name":"L Schneiden""#,
        r#""Lシュナイデン(214LL)","name":"214LL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214LM","name":"L Macht""#,
        r#""Lマハト(214LM)","name":"214LM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Lord's Strike""#,
        r#""M王者の行進(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214MH","name":"M Zerreissen""#,
        r#""Mツェアライセン(214MH)","name":"214MH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214ML","name":"M Schneiden""#,
        r#""Mシュナイデン(214ML)","name":"214ML""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214MM","name":"M Macht""#,
        r#""Mマハト(214MM)","name":"214MM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Zerreissen""#,
        r#""Uツァライセン(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Träumerei""#,
        r#""Hトロイメライ(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Träumerei""#,
        r#""Lトロイメライ(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Träumerei""#,
        r#""Mトロイメライ(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Träumerei""#,
        r#""Uトロイメライ(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Anzünden""#,
        r#""Hアン・ツュンデン(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Anzünden""#,
        r#""Lアン・ツュンデン(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Anzünden""#,
        r#""Mアン・ツュンデン(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Anzünden""#,
        r#""Uアン・ツュンデン(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Platzen""#,
        r#""Hプラッツェン(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Platzen""#,
        r#""Lプラッツェン(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Platzen""#,
        r#""Mプラッツェン(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Platzen""#,
        r#""Uプラッツェン(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Lohenwolf""#,
        r#""ローエン・ヴォルフ(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Feuerrote Krone""#,
        r#""ファイアーローター・クローネ(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"X-Seele""#,
        r#""イクスゼーレ(5U)","name":"5U""#,
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
