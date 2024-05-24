use crate::{MoveInfo, CHARS};
use regex::Regex;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
extern crate ureq;

#[derive(Deserialize, Debug)]
struct Response {
    cargoquery: Vec<Data>,
}

#[derive(Deserialize, Debug)]
struct Data {
    title: Title,
}

#[derive(Deserialize, Debug)]
struct Title {
    input: Option<String>,
    name: Option<String>,
    damage: Option<String>,
    guard: Option<String>,
    invuln: Option<String>,
    startup: Option<String>,
    active: Option<String>,
    recovery: Option<String>,
    #[serde(rename = "onHit")]
    hit: Option<String>,
    #[serde(rename = "onBlock")]
    block: Option<String>,
    // level: Option<String>,
    // counter: Option<String>,
}

pub async fn frames_to_json(
    mut char_page_response_json: String,
    mut file: &File,
    char_count: usize,
) {
    char_page_response_json = char_page_response_json.replace(r#" &gt; "#, "");
    char_page_response_json = char_page_response_json.replace(r#"&lt;br&gt;"#, ", ");
    char_page_response_json = char_page_response_json.replace(r#"&lt;br/&gt;"#, ", ");
    // Ino low profile
    char_page_response_json = char_page_response_json.replace(r#" &lt;span class=&quot;tooltip&quot; &gt;Low Profile&lt;span class=&quot;tooltiptext&quot; style=&quot;&quot;&gt;When a character's hurtbox is entirely beneath an opponent's attack. This can be caused by crouching, certain moves, and being short.&lt;/span&gt;&lt;/span&gt;"#, "");

    char_page_response_json = char_page_response_json.replace(r#"c."#, "近");
    char_page_response_json = char_page_response_json.replace(r#"f."#, "遠");
    char_page_response_json = char_page_response_json.replace(r#"j."#, "j");
    char_page_response_json = char_page_response_json.replace(r#"Ultimate"#, "U");
    char_page_response_json = char_page_response_json.replace(r#"Ulitmate"#, "U");
    char_page_response_json = char_page_response_json.replace(
        r#""BC","name":"Brave Counter""#,
        r#""ブレイブカウンター","name":"ブレイブカウンター""#,
    );
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

    // 2B
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
        r#""5U~U","name":"Counterstrike""#,
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
        r#""jU~U","name":"Grab Pod Follow-up""#,
        r#""ポッド掴まり追加攻撃(jUU)","name":"jUU""#,
    );
    // アニラ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Flock to the Future""#,
        r#""H前途羊羊(214H)","name":"214H""#,
    );
    println!("{}", char_page_response_json);
    char_page_response_json = char_page_response_json.replace(
        r#""214H~4X","name":"H And Beyond!""#,
        r#""H意気羊羊(214H4X)","name":"214H4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Flock to the Future""#,
        r#""L前途羊羊(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L~4X","name":"L And Beyond!""#,
        r#""L意気羊羊(214L4X)","name":"214L4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Flock to the Future""#,
        r#""M前途羊羊(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M~4X","name":"M And Beyond!""#,
        r#""M意気羊羊(214M4X)","name":"214M4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Fluff 'Em, Stuff 'Em!""#,
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
        r#""5U~U","name":"Ramification Follow-up""#,
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

    /*
    // グランの技名
    re = Regex::new(r#""input":"(214)([LMHU])","name":"[LMHU] Overdrive Surge""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2ドラブバースト($1$2)","name":"$1$2""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(236)([LMHU])","name":"[LMHU] Reginleiv""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2レギンレイヴ($1$2)","name":"$1$2""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(623)([LMHU])","name":"[LMHU] Rising Sword""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2ライジングソード($1$2)","name":"$1$2""#,
        )
        .to_string();

    char_page_response_json = char_page_response_json.replace(
        r#""214L~214M","name":"L Overdrive Surge Followup""#,
        r#""Lドライブバースト追加攻撃(214L214M)","name":"L214M214""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Tempest Blade""#,
        r#""テンペストブレード(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H236U","name":"Eternal Edge""#,
        r#""比類無き十の力(236236H236U)","name":"236236H236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Catastrophe""#,
        r#""カタストロフィ(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Catastrophe""#,
        r#""カタストロフィ(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5[U]","name":"Power Raise""#,
        r#""パワーレイズ(5[U])","name":"5[U]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5[U]~X","name":"Cancel""#,
        r#""パワーレイズキャンセル([5U]X)","name":"5[U]X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5[U]~]U[","name":"Power Raise (Attack)""#,
        r#""パワーレイズアタック(5[U]]U[)","name":"5[U]]U[""#,
    );

    re = Regex::new(r#""input":"(214)([LMHU])","name":"[LMHU] Vorpal Blade""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2ボーパルブレード($1$2)","name":"$1$2""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(236\[)([LMH])(\])","name":"[LMH] Reginleiv: Recidive""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2レギンレイヴ・レシディーヴ($1$2$3)","name":"$1$2$3""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(214)([LM])~(214[LM])","name":"[LM] Vorpal Blade Follow-up""#)
        .unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2ボーパルブレード（追加攻撃）($1$2$3)","name":"$1$2$3""#,
        )
        .to_string();

    re = Regex::new(r#""input":"(5U )([lL]v[0-4])","name":"Overdrive Surge""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2ドライブバースト($1$2)","name":"$1$2""#,
        )
        .to_string();

    char_page_response_json = char_page_response_json.replace(
        r#""214H~214H","name":"H Vorpal Blade Follow-up 1""#,
        r#""ボーパルブレード（追加攻撃１）(214H214H)","name":"214H214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214H~214H~214H","name":"H Vorpal Blade Follow-up 2""#,
        r#""ボーパルブレード（追加攻撃２）(214H214H214H)","name":"214H214H214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Eternal Ascendancy""#,
        r#""絶類なる十の力(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Skyfall""#,
        r#""大いなる破局(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5\[U\] ~ X","name":"Cancel""#,
        r#""ドライブバースト（キャンセル）(5[U]X)","name":"5[U]X""#,
    );

    // ニーアの技名置換
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
    */

    let mut moves_info: Response = serde_json::from_str(&char_page_response_json).unwrap();

    for x in 0..moves_info.cargoquery.len() {
        // Replacing None values with a generic '-'
        if moves_info.cargoquery[x].title.input.is_none() {
            moves_info.cargoquery[x].title.input = Some("-".to_string());
        } else {
            // Skips finish blow for sol
            if *moves_info.cargoquery[x].title.input.as_ref().unwrap() == "j.XX during Homing Jump"
            {
                continue;
            }
        }
        if moves_info.cargoquery[x].title.name.is_none() {
            moves_info.cargoquery[x].title.name = Some(
                moves_info.cargoquery[x]
                    .title
                    .input
                    .as_ref()
                    .unwrap()
                    .to_string(),
            );
        } else {
            // Skips dash cancel entry, ino hoverdash chipp escape zato flight and finish blow
            if *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Dash Cancel"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Hoverdash"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Finish Blow"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Flight"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Escape"
            {
                continue;
            }
        }
        if moves_info.cargoquery[x].title.damage.is_none() {
            moves_info.cargoquery[x].title.damage = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.guard.is_none() {
            moves_info.cargoquery[x].title.guard = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.invuln.is_none() {
            moves_info.cargoquery[x].title.invuln = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.startup.is_none() {
            moves_info.cargoquery[x].title.startup = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.active.is_none() {
            moves_info.cargoquery[x].title.active = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.recovery.is_none() {
            moves_info.cargoquery[x].title.recovery = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.hit.is_none() {
            moves_info.cargoquery[x].title.hit = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.hit.is_none() {
            moves_info.cargoquery[x].title.hit = Some("-".to_string());
        }
        if let Some(input) = &mut moves_info.cargoquery[x].title.input {
            *input = input.replace("~", "");
            // *input = input.replace(" ", "");
        }
        if let Some(hit) = &mut moves_info.cargoquery[x].title.hit {
            *hit = hit.replace("&lt;span style=&quot;color: \n#b70c0b&quot; &gt;", "");
            *hit = hit.replace("&lt;span style=&quot;color: \n#00d7c0&quot; &gt;'''", "");
            *hit = hit.replace("'''&lt;/span&gt;", "");
            *hit = hit.replace("'''", "");
            *hit = hit.replace("&lt;span style=&quot;color: Tomato&quot; &gt;", "");
        }
        if moves_info.cargoquery[x].title.block.is_none() {
            moves_info.cargoquery[x].title.block = Some("-".to_string());
        }
        if let Some(block) = &mut moves_info.cargoquery[x].title.block {
            *block = block.replace("&lt;span style=&quot;color: \n#b70c0b&quot; &gt;", "");
            *block = block.replace("&lt;span style=&quot;color: \n#00d7c0&quot; &gt;'''", "");
            *block = block.replace("'''&lt;/span&gt;", "");
            *block = block.replace("'''", "");
            *block = block.replace("&lt;span style=&quot;color: Tomato&quot; &gt;", "");
        }
        // if moves_info.cargoquery[x].title.level.is_none(){
        //   moves_info.cargoquery[x].title.level = Some("-".to_string());
        //}
        //        if moves_info.cargoquery[x].title.riscgain.is_none(){
        //            moves_info.cargoquery[x].title.riscgain = Some("-".to_string());
        //        }
        //        if moves_info.cargoquery[x].title.prorate.is_none(){
        //            moves_info.cargoquery[x].title.prorate = Some("-".to_string());
        //        }
        //        if moves_info.cargoquery[x].title.counter.is_none(){
        //            moves_info.cargoquery[x].title.counter = Some("-".to_string());
        //        }

        println!("");
        println!(
            "{}",
            moves_info.cargoquery[x]
                .title
                .input
                .as_ref()
                .unwrap()
                .to_string()
        );
        // Serializing frame data
        let processed_moves_info = serde_json::to_string(&MoveInfo {
            input: moves_info.cargoquery[x]
                .title
                .input
                .as_ref()
                .unwrap()
                .to_string(),
            name: moves_info.cargoquery[x]
                .title
                .name
                .as_ref()
                .unwrap()
                .to_string(),
            damage: moves_info.cargoquery[x]
                .title
                .damage
                .as_ref()
                .unwrap()
                .to_string(),
            guard: moves_info.cargoquery[x]
                .title
                .guard
                .as_ref()
                .unwrap()
                .to_string(),
            invincibility: moves_info.cargoquery[x]
                .title
                .invuln
                .as_ref()
                .unwrap()
                .to_string(),
            startup: moves_info.cargoquery[x]
                .title
                .startup
                .as_ref()
                .unwrap()
                .to_string(),
            active: moves_info.cargoquery[x]
                .title
                .active
                .as_ref()
                .unwrap()
                .to_string(),
            recovery: moves_info.cargoquery[x]
                .title
                .recovery
                .as_ref()
                .unwrap()
                .to_string(),
            hit: moves_info.cargoquery[x]
                .title
                .hit
                .as_ref()
                .unwrap()
                .to_string(),
            block: moves_info.cargoquery[x]
                .title
                .block
                .as_ref()
                .unwrap()
                .to_string(),
            // level: moves_info.cargoquery[x].title.level.as_ref().unwrap().to_string(),
            // riscgain: moves_info.cargoquery[x].title.riscgain.as_ref().unwrap().to_string(),
            // scaling: moves_info.cargoquery[x].title.prorate.as_ref().unwrap().to_string(),
            // counter: moves_info.cargoquery[x].title.counter.as_ref().unwrap().to_string(),
        })
        .unwrap();

        write!(file, "{}", processed_moves_info)
            .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count] + ".json'."));

        // Skip writting comma/tab if next and last iteration
        // contains 'finish blow' in last the input field
        if x == moves_info.cargoquery.len() - 2
            && *moves_info.cargoquery[x + 1].title.input.as_ref().unwrap()
                == "j.XX during Homing Jump"
        {
            continue;
        } else if x != moves_info.cargoquery.len() - 1 {
            // Adding comma/tab
            // file.write(b",\n\t")
            //     .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS[char_count]+ ".json'."));
            (&mut file).write_all(b",\n\t").expect(
                &("\nFailed to write ',\\n\\t' while serializing '".to_owned()
                    + CHARS[char_count]
                    + ".json'."),
            );
        }
    }
}
