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
    char_page_response_json =
        char_page_response_json.replace(r#"&lt;span style=&quot;color: \n#3455ff&quot; &gt;"#, "");
    char_page_response_json =
        char_page_response_json.replace(r#"&lt;span style=&quot;color: \n#bcbd0c&quot; &gt"#, "");

    char_page_response_json = char_page_response_json.replace(r#"&#039;"#, "");
    char_page_response_json = char_page_response_json.replace(r#"&lt;/span&gt;"#, "");
    char_page_response_json =
        char_page_response_json.replace(r#"&lt;span style=&quot;color: \n#00d7c0&quot; &gt;"#, "");

    char_page_response_json = char_page_response_json.replace(r#"&gt;"#, "");
    char_page_response_json =
        char_page_response_json.replace(r#"&lt;span class=&quot;tooltip&quot;"#, "");
    char_page_response_json =
        char_page_response_json.replace(r#"&lt;span class=&quot;tooltiptext&quot;"#, "");
    char_page_response_json = char_page_response_json.replace(r#"&lt;/span"#, "");

    char_page_response_json =
        char_page_response_json.replace(r#""input":"5L""#, r#""input":"近L""#);
    char_page_response_json = char_page_response_json.replace(r#"c."#, "近");
    char_page_response_json = char_page_response_json.replace(r#"f."#, "遠");
    char_page_response_json = char_page_response_json.replace(r#"j."#, "j");
    char_page_response_json = char_page_response_json.replace(r#"Ultimate"#, "U");
    char_page_response_json = char_page_response_json.replace(r#"Ulitmate"#, "U");
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

    // グラン
    re = Regex::new(r#""input":"(214)([LMHU])","name":"[LMHU] Overdrive Surge""#).unwrap();
    char_page_response_json = re
        .replace_all(
            &char_page_response_json,
            r#""input":"$2ドライブバースト($1$2)","name":"$1$2""#,
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
        r#""Lドライブバースト追加攻撃(214L214M)","name":"214L214M""#,
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
        r#""L/M軌道変化(236X4X)","name":"236X4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X6X","name":"L/M Wind Shift""#,
        r#""L/M軌道変化(236X6X)","name":"236X6X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X8X","name":"L/M Wind Shift""#,
        r#""L/M軌道変化(236X8X)","name":"236X8X""#,
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
        r#""Hオービタル・レゾナンス追加攻撃(5U~H)","name":"5UH""#,
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

    // シス
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Gate of Demons""#,
        r#""H鬼門・修羅(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Gate of Demons""#,
        r#""L鬼門・修羅(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Gate of Demons""#,
        r#""M鬼門・修羅(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Gate of Demons""#,
        r#""U鬼門・修羅(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Six-Claw Execution""#,
        r#""H六爪無斬(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Six-Claw Execution""#,
        r#""L六爪無斬(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Six-Claw Execution""#,
        r#""M六爪無斬(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Six-Claw Execution""#,
        r#""U六爪無斬(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X4L","name":"L Wolf's Howl""#,
        r#""L狼哮(236X4L)","name":"236X4L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X4M/H","name":"M/H Wolf's Howl""#,
        r#""M/H狼哮(236X4M/H)","name":"236X4M/H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X6L","name":"L/H Archdemon Kick""#,
        r#""L/H真鬼蹴(236X6L)","name":"236X6L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X6M/H","name":"M Archdemon Kick""#,
        r#""M真鬼蹴(236X6M/H)","name":"236X6M/H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X6U/4U","name":"Shadow Trace""#,
        r#""残影陣(236X6U/4U)","name":"236X6U/4U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X8L","name":"L/H Snake Bite""#,
        r#""L/H蛇頭斬(236X8L)","name":"236X8L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X8M/H","name":"M Snake Bite""#,
        r#""M蛇頭斬(236X8M/H)","name":"236X8M/H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Thunderflash""#,
        r#""H迅門・紫電(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Thunderflash""#,
        r#""L迅門・紫電(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Thunderflash""#,
        r#""M迅門・紫電(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Thunderflash""#,
        r#""U迅門・紫電(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214H","name":"Air H Gate of Demons""#,
        r#""空中H鬼門・修羅(j214H)","name":"j214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214L","name":"Air L Gate of Demons""#,
        r#""空中L鬼門・修羅(j214L)","name":"j214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214M","name":"Air M Gate of Demons""#,
        r#""空中M鬼門・修羅(j214M)","name":"j214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j214U","name":"U Aerial Gate of Demons""#,
        r#""空中U鬼門・修羅(j214U)","name":"j214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8H","name":"H Empty Hand""#,
        r#""H虚空拳([2]8H)","name":"[2]8H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8L","name":"L Empty Hand""#,
        r#""L虚空拳([2]8L)","name":"[2]8L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8M","name":"M Empty Hand""#,
        r#""M虚空拳([2]8M)","name":"[2]8M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[2]8U","name":"U Empty Hand""#,
        r#""U虚空拳([2]8U)","name":"[2]8U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214214H","name":"Six-Ruin's Enlightenment""#,
        r#""六崩の悟り(214214H)","name":"214214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Void Claws: Terminus""#,
        r#""天地虚空夜叉閃刃(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Three Thousand and One Talons""#,
        r#""三千一穿明王波斬(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json
        .replace(r#""5U","name":"Demon Step""#, r#""闘鬼陣(5U)","name":"5U""#);
    char_page_response_json = char_page_response_json.replace(
        r#""5[U]","name":"New Perspective""#,
        r#""転地(5[U])","name":"5[U]""#,
    );

    // ジークフリート
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Verdrängen""#,
        r#""Hメサジェ(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Verdrängen""#,
        r#""Lメサジェ(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Verdrängen""#,
        r#""Mメサジェ(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Verdrängen""#,
        r#""Uメサジェ(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Orkan""#,
        r#""Hオルカーン(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Orkan""#,
        r#""Lオルカーン(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Orkan""#,
        r#""Mオルカーン(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Orkan""#,
        r#""Uオルカーン(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~H","name":"Uwe""#,
        r#""ウーヴェ(22XH)","name":"22XH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~L","name":"Héritier""#,
        r#""エリティエ(22XL)","name":"22XL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~M","name":"L'Ombre d'Hier""#,
        r#""ロンブル・ディエール(22XM)","name":"22XM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~U","name":"Deliverance""#,
        r#""デリヴランス(22XU)","name":"22XU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Nelah Nav""#,
        r#""Hネラナヴ(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Nelah Nav""#,
        r#""Lネラナヴ(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Nelah Nav""#,
        r#""Mネラナヴ(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Nelah Nav""#,
        r#""Uネラナヴ(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Messager""#,
        r#""Hメサジェ(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Messager""#,
        r#""Lメサジェ(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Messager""#,
        r#""Mメサジェ(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Messager""#,
        r#""Uメサジェ(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214214H","name":"Blood of the Dragon""#,
        r#""ドラゴンブラッド(214214H)","name":"214214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Schwarze Fänge""#,
        r#""シュヴァルツ・ファング(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Rasendes Scharzblut""#,
        r#""ラーゼライシュヴァルツブルート(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Manigance""#,
        r#""マニガンス(5U)","name":"5U""#,
    );

    // ソリッズ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Punch the Stars""#,
        r#""H剛破天衝(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Punch the Stars""#,
        r#""L剛破天衝(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Punch the Stars""#,
        r#""M剛破天衝(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Punch the Stars""#,
        r#""U剛破天衝(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Rock Smash""#,
        r#""Hロックスマッシュ(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Rock Smash""#,
        r#""Lロックスマッシュ(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Rock Smash""#,
        r#""Mロックスマッシュ(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Rock Smash""#,
        r#""Uロックスマッシュ(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Impact Knuckles""#,
        r#""Hインパクトナックル(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Impact Knuckles""#,
        r#""Lインパクトナックル(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Impact Knuckles""#,
        r#""Mインパクトナックル(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Impact Knuckles""#,
        r#""Uインパクトナックル(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Roundhouse Fang""#,
        r#""H流牙蹴(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Roundhouse Fang""#,
        r#""L流牙蹴(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Roundhouse Fang""#,
        r#""M流牙蹴(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Roundhouse Fang""#,
        r#""U流牙蹴(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Macho 214U","name":"Macho U Punch the Stars""#,
        r#""漢気アルティメイアム剛破天衝(Macho 214U)","name":"Macho 214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214X~214X","name":"Tenacious Will""#,
        r#""不撓不屈(214X~214X)","name":"214X~214X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214X~236X","name":"Bravado Bullet""#,
        r#""肉弾気焔(214X~236X)","name":"214X~236X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Leaping Tiger, Charging Dragon""#,
        r#""破虎撃龍(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Macho Ultimatum""#,
        r#""漢気アルティメイアム(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Macho 236236U","name":"Way of the Fundoshi Fist""#,
        r#""拳褌軼敵・漢の正拳突き(Macho 236236U)","name":"Macho 236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Muscle Fury""#,
        r#""マッスルフューリー(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U~6X","name":"Reactive Muscles""#,
        r#""マッスルリアクション(5U6X)","name":"5U6X""#,
    );

    // バザラガ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Instinction""#,
        r#""Hインスティンクション(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Instinction""#,
        r#""Lインスティンクション(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Instinction""#,
        r#""Mインスティンクション(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Instinction""#,
        r#""Uインスティンクション(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Savage Rampage""#,
        r#""Hクルード・ランページ(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Savage Rampage""#,
        r#""Lクルード・ランページ(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Savage Rampage""#,
        r#""Mクルード・ランページ(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Savage Rampage""#,
        r#""Uクルード・ランページ(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X44/G","name":"Cancel""#,
        r#""キャンセル(22X44)","name":"22X44""#,
    );
    char_page_response_json = char_page_response_json
        .replace(r#""22X6","name":"March""#, r#""前進(22X6)","name":"22X6""#);
    char_page_response_json = char_page_response_json.replace(
        r#""22XH","name":"Crushing Strike""#,
        r#""縦斬り(22XH)","name":"22XH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22XL","name":"Rising Slash""#,
        r#""なぎ払い上(22XL)","name":"22XL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22XM","name":"Sweeping Slash""#,
        r#""なぎ払い下(22XM)","name":"22XM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22XU","name":"Ravaging Stomp""#,
        r#""突き立て(22XU)","name":"22XU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""632146H","name":"H Great Scythe Grynoth""#,
        r#""H大鎌グロウノス(632146H)","name":"632146H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""632146L","name":"L Great Scythe Grynoth""#,
        r#""L大鎌グロウノス(632146L)","name":"632146L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""632146M","name":"M Great Scythe Grynoth""#,
        r#""M大鎌グロウノス(632146M)","name":"632146M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""632146U","name":"U Great Scythe Grynoth""#,
        r#""U大鎌グロウノス(632146U)","name":"632146U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6H","name":"H Battalions of Fear""#,
        r#""Hバタリアンズ・オブ・フィア([4]6H)","name":"[4]6H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6L","name":"L Battalions of Fear""#,
        r#""Lバタリアンズ・オブ・フィア([4]6L)","name":"[4]6L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6M","name":"M Battalions of Fear""#,
        r#""Mバタリアンズ・オブ・フィア([4]6M)","name":"[4]6M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""[4]6U","name":"U Battalions of Fear""#,
        r#""Uバタリアンズ・オブ・フィア([4]6U)","name":"[4]6U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Bloody Moon""#,
        r#""ブラッディ・ムーン(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Aftermath""#,
        r#""アフターマス(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Soul Forge""#,
        r#""ソウルフォージ(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Samsara""#,
        r#""H輪廻(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Samsara""#,
        r#""L輪廻(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Samsara""#,
        r#""M輪廻(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Samsara""#,
        r#""U輪廻(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214X~U","name":"Yama""#,
        r#""夜摩天(214XU)","name":"214XU""#,
    );
    char_page_response_json = char_page_response_json
        .replace(r#""22H","name":"H Anitya""#, r#""H無常(22H)","name":"22H""#);
    char_page_response_json = char_page_response_json
        .replace(r#""22L","name":"L Anitya""#, r#""L無常(22L)","name":"22L""#);
    char_page_response_json = char_page_response_json
        .replace(r#""22M","name":"M Anitya""#, r#""M無常(22M)","name":"22M""#);
    char_page_response_json = char_page_response_json
        .replace(r#""22U","name":"U Anitya""#, r#""U無常(22U)","name":"22U""#);
    char_page_response_json = char_page_response_json.replace(
        r#""22X~U","name":"Trayastrimsha""#,
        r#""忉利天(22XU)","name":"22XU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Deadly Flare""#,
        r#""H光焔(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Deadly Flare""#,
        r#""L光焔(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Deadly Flare""#,
        r#""M光焔(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Deadly Flare""#,
        r#""U光焔(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236X~U","name":"Nirmanarati""#,
        r#""化楽天(236XU)","name":"236XU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Nimbus""#,
        r#""H身光(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Nimbus""#,
        r#""L身光(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Nimbus""#,
        r#""M身光(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Nimbus""#,
        r#""U身光(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623X~U","name":"Tushita""#,
        r#""兜率天(623XU)","name":"623XU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Yomotsu""#,
        r#""黄泉(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Cataclysm""#,
        r#""神撃(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Celestial Dominion""#,
        r#""他化自在(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U~U","name":"Celestial Strike""#,
        r#""四天王(5UU)","name":"5UU""#,
    );
    // ビカラ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Ring the Dormouse""#,
        r#""Hグリーティング・ドーマウス(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Ring the Dormouse""#,
        r#""Lグリーティング・ドーマウス(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Ring the Dormouse""#,
        r#""Mグリーティング・ドーマウス(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Ring the Dormouse""#,
        r#""Uグリーティング・ドーマウス(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Marching Teeth""#,
        r#""Hマーチング・ディース(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Marching Teeth""#,
        r#""Lマーチング・ディース(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Marching Teeth""#,
        r#""Mマーチング・ディース(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Marching Teeth""#,
        r#""Uマーチング・ディース(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Dream Attraction""#,
        r#""Hドリーム・アトラクション(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Dream Attraction""#,
        r#""Lドリーム・アトラクション(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Dream Attraction""#,
        r#""Mドリーム・アトラクション(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Dream Attraction""#,
        r#""Uドリーム・アトラクション(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236XU","name":"Fire!""#,
        r#""シュート！(236XU)","name":"236XU""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Rodent Rhythm""#,
        r#""Hビート・ザ・マウス(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Rodent Rhythm""#,
        r#""Lビート・ザ・マウス(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Rodent Rhythm""#,
        r#""Mビート・ザ・マウス(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Rodent Rhythm""#,
        r#""Uビート・ザ・マウス(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json
        .replace(r#""Ball","name":"Ball""#, r#""ボール(Ball)","name":"Ball""#);
    char_page_response_json =
        char_page_response_json.replace(r#""Bomb","name":"Bomb""#, r#""爆弾","name":"Bomb""#);
    char_page_response_json = char_page_response_json.replace(
        r#""Cannonball","name":"Cannonball""#,
        r#""鉄球","name":"Cannonball""#,
    );
    char_page_response_json = char_page_response_json
        .replace(r#""Cheese","name":"Cheese""#, r#""チーズ","name":"Cheese""#);
    char_page_response_json =
        char_page_response_json.replace(r#""Heart","name":"Heart""#, r#""ハート","name":"Heart""#);
    char_page_response_json = char_page_response_json.replace(
        r#""Smoke Bomb","name":"Smoke Bomb""#,
        r#""煙玉","name":"Smoke Bomb""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Spike Ball","name":"Spike Ball""#,
        r#""トゲ鉄球","name":"Spike Ball""#,
    );
    char_page_response_json =
        char_page_response_json.replace(r#""Star","name":"Star""#, r#""スター","name":"Star""#);
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Eccentrical Parade""#,
        r#""エキセントリックパレード(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Dream Come True""#,
        r#""ドリーム・チョイス(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5UX","name":"Sike!""#,
        r#""ストップ！(5UX)","name":"5UX""#,
    );
    // ヴィーラ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Scarlet Oath""#,
        r#""Hスカーレット・オース(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Scarlet Oath""#,
        r#""Lスカーレット・オース(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Scarlet Oath""#,
        r#""Mスカーレット・オース(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Scarlet Oath""#,
        r#""Uスカーレット・オース(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Summon Luminiera""#,
        r#""Hシュヴァリエ召喚(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Summon Luminiera""#,
        r#""Lシュヴァリエ召喚(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Summon Luminiera""#,
        r#""Mシュヴァリエ召喚(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Summon Luminiera""#,
        r#""Uシュヴァリエ召喚(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Darkane""#,
        r#""Hダーケイン(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Darkane""#,
        r#""Lダーケイン(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Darkane""#,
        r#""Mダーケイン(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Darkane""#,
        r#""Uダーケイン(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Red Beryl Sword""#,
        r#""Hレッドベリルソード(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Red Beryl Sword""#,
        r#""Lレッドベリルソード(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Red Beryl Sword""#,
        r#""Mレッドベリルソード(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Red Beryl Sword""#,
        r#""Uレッドベリルソード(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 22H","name":"H Summon Luminiera (Luminiera Form)""#,
        r#""変身後Hシュヴァリエ召喚(g22H)","name":"g22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 22U","name":"U Grand Summon Luminiera""#,
        r#""変身後Uシュヴァリエ召喚(g22U)","name":"g22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 236U","name":"U Grand Darkane""#,
        r#""変身後Uダーケイン(g236U)","name":"g236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 236[H]","name":"H Darkane (Charged, Luminiera Form)""#,
        r#""変身後Hダーケイン（タメ）(g236[H])","name":"g236[H]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 236[M]","name":"M Darkane (Charged, Luminiera Form)""#,
        r#""変身後Mダーケイン（タメ）(g236[M])","name":"g236[M]""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 623H","name":"H Red Beryl Sword (Luminiera Form)""#,
        r#""変身後Hレッドベリルソード(g623H)","name":"g623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 623M","name":"M Red Beryl Sword (Luminiera Form)""#,
        r#""変身後Mレッドベリルソード(g623M)","name":"g623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 623U","name":"U Grand Red Beryl Sword""#,
        r#""変身後Uレッドベリルソード(g623U)","name":"g623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Luminiera Merge""#,
        r#""シュヴァリエマージ(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Affection Abyss""#,
        r#""アフェクション・アビス(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 236236H","name":"Punishment Ray""#,
        r#""パニッシュメント・レイ(g236236H)","name":"g236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 236236U","name":"Iliad Vision""#,
        r#""イーリアス・ヴィジョン(g236236U)","name":"Grand 236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Blade of Light""#,
        r#""光の剣(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 5U","name":"Graveview""#,
        r#""グレースビュー(変身後5U)","name":"g5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 5U~2","name":"Land""#,
        r#""降りる(5U2)","name":"5U2""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 5U~4","name":"Retreat""#,
        r#""後退(5U4)","name":"5U4""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 5U~6","name":"Advance""#,
        r#""前進(5U6)","name":"5U6""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 5U~8","name":"Jump""#,
        r#""ジャンプ(5U8)","name":"5U8""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand 5U~X","name":"Falling Strike""#,
        r#""下降攻撃(5UX)","name":"5UX""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""Grand j66","name":"Air Dash""#,
        r#""空中ダッシュ(j66)","name":"j66""#,
    );

    // ユエル
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Hanaarashi (1)""#,
        r#""H華嵐(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214H-M","name":"H Hanaarashi (2)""#,
        r#""HM華嵐(214H-M)","name":"214H-M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Hanaarashi""#,
        r#""L華嵐(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Hanaarashi""#,
        r#""M華嵐(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Hanaarashi""#,
        r#""U華嵐(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Foxflame""#,
        r#""H狐火(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Foxflame""#,
        r#""L狐火(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Foxflame""#,
        r#""M狐火(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Foxflame""#,
        r#""U狐火(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Starlit Sky""#,
        r#""H夜天光(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Starlit Sky""#,
        r#""L夜天光(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Starlit Sky""#,
        r#""M夜天光(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Starlit Sky""#,
        r#""U夜天光(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Hien: Homuragaeshi""#,
        r#""H飛燕・焔返し(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Hien: Homuragaeshi""#,
        r#""L飛燕・焔返し(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Hien: Homuragaeshi""#,
        r#""M飛燕・焔返し(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Hien: Homuragaeshi""#,
        r#""U飛燕・焔返し(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Crimson Dance: Rinnen-aratame""#,
        r#""C紅之舞・凜炎改(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Sapphire Dance: Gentiana""#,
        r#""S蒼紅之舞・竜胆(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Third Dance""#,
        r#""参之毎(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json
        .replace(r#""td.2H","name":"Yugetsu""#, r#""融月(k2H)","name":"k2H""#);
    char_page_response_json =
        char_page_response_json.replace(r#""td.2L","name":null"#, r#""構え2L(k2L)","name":"k2L""#);
    char_page_response_json =
        char_page_response_json.replace(r#""td.2M","name":null"#, r#""構え2M(k2M)","name":"k2M""#);
    char_page_response_json = char_page_response_json.replace(
        r#""td.44","name":null"#,
        r#""構え後ろステップ(k44)","name":"k44""#,
    );
    char_page_response_json = char_page_response_json
        .replace(r#""td.5H","name":"Gurren""#, r#""紅蓮(k5H)","name":"k5H""#);
    char_page_response_json =
        char_page_response_json.replace(r#""td.5L","name":null"#, r#""構え5L(k5L)","name":"k5L""#);
    char_page_response_json =
        char_page_response_json.replace(r#""td.5M","name":null"#, r#""構え5M(k5M)","name":"k5M""#);
    char_page_response_json = char_page_response_json.replace(
        r#""td.66","name":null"#,
        r#""構え前ステップ(k66)","name":"k66""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""td.Catch","name":"Eye of the Sparrow""#,
        r#""燕返し(k当て身)","name":"k当て身""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""td.L+M","name":null"#,
        r#""構え投げ(k投げ)","name":"k投げ""#,
    );
    char_page_response_json =
        char_page_response_json.replace(r#""td.U","name":null"#, r#""構え解除(kU)","name":"kU""#);

    // ゼタ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Rhapsody""#,
        r#""Hラプソディー(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Rhapsody""#,
        r#""Lラプソディー(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Rhapsody""#,
        r#""Mラプソディー(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Rhapsody""#,
        r#""Uラプソディー(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214XH","name":"Knee Assault""#,
        r#""膝蹴り(214XH)","name":"214XH""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214XL","name":"Crimson Cleave""#,
        r#""なぎ払い(214XL)","name":"214XL""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214XM","name":"Rising Split""#,
        r#""切り上げ(214XM)","name":"214XM""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Spear of Arvess (Rise)""#,
        r#""Hアルベスの槍・上昇(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Spear of Arvess (Rise)""#,
        r#""Lアルベスの槍・上昇(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Spear of Arvess (Rise)""#,
        r#""Mアルベスの槍・上昇(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Spear of Arvess (Rise)""#,
        r#""Uアルベスの槍・上昇(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~1X/2X/3X","name":"Spear of Arvess (Followup)""#,
        r#""アルベスの槍・上昇（追加攻撃・下）(22X1X/2X/3X)","name":"22X1X/2X/3X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~6X/4X","name":"Spear of Arvess (Followup)""#,
        r#""アルベスの槍・上昇（追加攻撃・横）(22X6X/4X)","name":"22X6X/4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22X~9X/8X/7X","name":"Spear of Arvess (Followup)""#,
        r#""アルベスの槍・上昇（追加攻撃・上）(22X9X/8X/7X)","name":"22X9X/8X/7X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"H Infinite Wonders""#,
        r#""Hインフィニット・ワンダーズ(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"L Infinite Wonders""#,
        r#""Lインフィニット・ワンダーズ(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"M Infinite Wonders""#,
        r#""Mインフィニット・ワンダーズ(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Infinite Wonders""#,
        r#""Uインフィニット・ワンダーズ(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Spear of Arvess""#,
        r#""Hアルベスの槍(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Spear of Arvess""#,
        r#""Lアルベスの槍(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Spear of Arvess""#,
        r#""Mアルベスの槍(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Spear of Arvess""#,
        r#""Uアルベスの槍(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623X~4X/6X","name":"Spear of Arvess (Follow-up)""#,
        r#""アルベスの槍（追加攻撃・横）(623X4X/6X)","name":"623X4X/6X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623X~7X/9X","name":"Spear of Arvess (Followup)""#,
        r#""アルベスの槍（追加攻撃・斜め上）(623X7X/9X)","name":"623X7X/9X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623X~8X","name":"Spear of Arvess (Followup)""#,
        r#""アルベスの槍（追加攻撃・真上）(623X8X)","name":"623X8X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22H","name":"H Spear of Arvess (Fall)""#,
        r#""Hアルベスの槍・下降(j22H)","name":"j22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22L","name":"L Spear of Arvess (Fall)""#,
        r#""Lアルベスの槍・下降(j22L)","name":"j22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22M","name":"M Spear of Arvess (Fall)""#,
        r#""Mアルベスの槍・下降(j22M)","name":"j22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22U","name":"U Aerial Spear of Arvess""#,
        r#""Uアルベスの槍・下降(j22U)","name":"j22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""~6X/4X","name":"Spear of Arvess (Followup)""#,
        r#""アルベスの槍・下降（追加攻撃横）(6X/4X)","name":"6X/4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""~9X/8X/7X","name":"Spear of Arvess (Followup)""#,
        r#""アルベスの槍・下降（追加攻撃上）(9X/8X/7X)","name":"9X/8X/7X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""~j1X/j2X/j3X","name":"Spear of Arvess (Followup)""#,
        r#""アルベスの槍・下降（追加攻撃下）(j1X/j2X/j3X)","name":"j1X/j2X/j3X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""~j6X/j4X","name":"Spear of Arvess (Followup)""#,
        r#""アルベスの槍・下降（追加攻撃ジャンプ横）(j6X/j4X)","name":"j6X/j4X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Resolute Strike""#,
        r#""プロミネンスダイヴ(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Sirius Roar""#,
        r#""シリウスロア(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j236236H","name":"Resolute Strike""#,
        r#""空中プロミネンスダイヴ(j236236H)","name":"j236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Signo Drive""#,
        r#""シグノ・ドライブ(5U)","name":"5U""#,
    );

    // ゾーイ
    char_page_response_json = char_page_response_json.replace(
        r#""214H","name":"H Spinning Slash""#,
        r#""Hスピンスラッシュ(214H)","name":"214H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214H~5X","name":"H Ray Strike""#,
        r#""Hスピンスラッシュ（レイストライク）(214H5X)","name":"214H5X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214H~6X","name":"M Bisection""#,
        r#""Hスピンスラッシュ（バイセクション）(214H6X)","name":"214H6X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L","name":"L Spinning Slash""#,
        r#""Lスピンスラッシュ(214L)","name":"214L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L~5X","name":"L Ray Strike""#,
        r#""Lスピンスラッシュ（レイストライク）(214L5X)","name":"214L5X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214L~6X","name":"L Bisection""#,
        r#""Lスピンスラッシュ（バイセクション）(214L6X)","name":"214L6X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M","name":"M Spinning Slash""#,
        r#""Mスピンスラッシュ(214M)","name":"214M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M~5X","name":"M Ray Strike""#,
        r#""Mスピンスラッシュ（レイストライク）(214M5X)","name":"214M5X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214M~6X","name":"M Bisection""#,
        r#""Mスピンスラッシュ（バイセクション）(214M6X)","name":"214M6X""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""214U","name":"U Spinning Slash""#,
        r#""Uスピンスラッシュ(214U)","name":"214U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22H","name":"H Thunder""#,
        r#""Hサンダー(22H)","name":"22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22L","name":"L Thunder""#,
        r#""Lサンダー(22L)","name":"22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22M","name":"M Thunder""#,
        r#""Mサンダー(22M)","name":"22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""22U","name":"U Thunder""#,
        r#""Uサンダー(22U)","name":"22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236H","name":"Sweeping Beam""#,
        r#""コールワイバーン（スウィープスライド）(236H)","name":"236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236L","name":"Immortal Thrust""#,
        r#""コールワイバーン（イモータルスラスト）(236L)","name":"236L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236M","name":"Heavy Breath""#,
        r#""コールワイバーン（ブレス）(236M)","name":"236M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236U","name":"U Peacemaker's Wings""#,
        r#""調整の翼(236U)","name":"236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623H","name":"H Aerial Spinning Slash""#,
        r#""Hスピンスラッシュ・エア(623H)","name":"623H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623L","name":"L Aerial Spinning Slash""#,
        r#""Lスピンスラッシュ・エア(623L)","name":"623L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623M","name":"M Aerial Spinning Slash""#,
        r#""Mスピンスラッシュ・エア(623M)","name":"623M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""623U","name":"U Aerial Spinning Slash""#,
        r#""Uスピンスラッシュ・エア(623U)","name":"623U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22H","name":"H Thunder (Midair)""#,
        r#""空中Hサンダー(j22H)","name":"j22H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22L","name":"L Thunder (Midair)""#,
        r#""空中Lサンダー(j22L)","name":"j22L""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22M","name":"M Thunder (Midair)""#,
        r#""空中Mサンダー(j22M)","name":"j22M""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""j22U","name":"U Aerial Thunder""#,
        r#""空中Uサンダー(j22U)","name":"j22U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236H","name":"Gamma Ray""#,
        r#""ガンマ・レイ(236236H)","name":"236236H""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""236236U","name":"Armageddon""#,
        r#""アーマゲドン(236236U)","name":"236236U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U","name":"Convergence""#,
        r#""コンバージェンス(5U)","name":"5U""#,
    );
    char_page_response_json = char_page_response_json.replace(
        r#""5U~6X","name":"The Last Wish""#,
        r#""ラストウィッシュ(5U~6X)","name":"5U~6X""#,
    );
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
