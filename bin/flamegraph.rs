use mokuzu::Tokenizer;

const TEXT: &'static str = r#"与えられた検索式に従って、ウェブページ等を検索するサーバ、システムのこと。検索式は、最も単純な場合はキーワードとなる文字列のみであるが、複数のキーワードにAND（「かつ」、論理積）やOR（「または」、論理和）等の論理条件を組み合わせて指定することができるものが多い。
ロボット型検索エンジンの大きな特徴の一つとして、クローラ（ロボット・スパイダー）を用いることが挙げられる。このことにより、WWW上にある多数の情報を効率よく収集（日本の著作権法では複製）することができる。大規模な検索エンジンでは、80億ページ以上のページから検索が可能になっている。
収集したページの情報は、前もって解析し、索引情報（インデックス）を作成する（日本の著作権法では編集）。日本語などの言語では、自然言語処理機能が生成される索引の質に影響する。このため、多言語対応した検索エンジンの方が精度の高い検索が可能となる。
検索結果の表示順は、検索エンジンの質が最も問われる部分である。ユーザーが期待したページを検索結果の上位に表示することができなければ、ユーザーが離れてしまうからである。そのため、多くの検索エンジンが、表示順を決定するアルゴリズムを非公開にし、その性能を競っている。検索エンジン最適化業者の存在も、アルゴリズムを公開しない要因になっている。Googleは、そのアルゴリズムの一部であるPageRankを公開しているが、やはり、多くの部分が非公開になっている。Googleの場合、創設初期におけるアルゴリズムについては、創設者自身がウェブ上で公表している論文でその一端を知ることができる。 参照 英語原文[1]日本語の解説[2]
ウェブページの更新時刻の情報を用いて、新しい情報に限定して検索できるものや、検索結果をカテゴリ化して表示するものなど、特長のある機能を搭載したり、検索結果をユーザーへ最適化していく動きもある。
従来のウェブページを検索するだけの検索エンジンにとどまらず、最近ではインターネットショッピング専用の検索エンジンなど、特定の分野に特化した検索エンジンの開発も散見される。商品検索では、価格比較サービス日本最大手の価格.comや、ベンチャー企業が開発するQOOPIEなどある。また、職業検索エンジンとしてはCraigslistなどがある。 Google、Yahoo!、インフォシーク、テクノラティ、MARSFLAG、Altavista、ムーター、AlltheWeb、Teoma、WiseNut、Inktomi、SAGOOL、Yahoo! JAPAN (2005.10〜) など。
ディレクトリ型検索エンジン
人手で構築したウェブディレクトリ内を検索するサーバ、システムのこと。
人手で構築しているため、質の高いウェブサイトを検索可能。概要を人手で記入しているため、検索結果の一覧から目的のサイトを探しやすい、サイトのカテゴリ分けがされていることから、特定分野や地区などに限定したサイトを探しやすいという特長がある。
しかし、検索対象となるサイトは人手で入力するため、検索対象となるサイト数が多くできないという欠点がある。
インターネットが一般に使われるようになった初期（1990年代）のころには、ディレクトリ型が主体であったが、WWWの爆発的な拡大によって、あらゆるウェブサイトを即時にディレクトリに反映させることが事実上不可能になり、現在では主流ではなくなっている。 このため、ディレクトリ型検索エンジンでは、検索にヒットするサイトが無かった場合、ロボット型検索エンジンを用いて結果を表示するような、併用型のものが多い。
日立国際ビジネスのHole-in-One（〜2004年11月）、Yahoo!JAPANのYahoo!カテゴリ（〜2018年3月）、LookSmart Japan（〜2006年6月）、goo、Open Directory ProjectことDMOZ（～2017年3月）など。
分散型検索エンジン
P2P通信によってウェブコンテンツのインデックスを多数のピアに分散させ、P2Pネットワーク全体で各ピアの持つインデックスを共有する検索システムのこと。
ウェブのクロールは各ピアが独自に行い、インデクサーはRWI(Reverse Word Index)を作成する。作成されたインデックスの一部はDHT(分散ハッシュテーブル、Distributed Hash Table)として他のピアに分配される。
検索は自分のピアの端末からP2Pネットワーク上にある他のピアにリクエストを送信することにより行うことができる。
分散型検索エンジンの例としてはYaCyがある。YaCyは「人民による人民のためのウェブ検索」を標榜し、分散型であることにより検閲を防ぐことができるとしている。[3]
メタ検索エンジン
ひとつの検索ワードを複数の検索エンジンで検索することをメタ検索という（横断検索エンジンと呼ぶこともある）。 詳細は「メタ検索エンジン」を参照のこと。"#;

fn main() {
    let mut tokenizer = Tokenizer::for_search();
    for _ in 0..10_000 {
        tokenizer.tokenize(TEXT);
    }
}