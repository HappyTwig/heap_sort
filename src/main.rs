use rand::Rng;

fn main() {
    println!("random number!");

    const NUM: usize = 15;

    // 0 以上 100 以下のランダムな整数を 15 回発生、表示
    let mut a = [0;NUM];
    for i in 0..NUM {
        let random_num = rand::thread_rng().gen_range(0,101);
        a[i] = random_num;
        println!("a[{}]:{}", i, a[i]);
    }

    // ヒープソートで降順に並べ替え
    // サイズNUMの二分ヒープを作成
    let mut size;   // 二分ヒープに追加済みのデータの個数

    // 二分ヒープのデータ個数を0にする
    size = 0;

    // sizeがソートするデータの個数になるまで二分ヒープにデータ追加
    while size < NUM {
        // a[size]を二分ヒープに追加
        let mut add;    // 追加ノードの位置
        let mut parent; // 追加ノードの親の位置

        // まだ二分ヒープに追加していないデータの先頭を二分ヒープに追加
        add = size;
        if add == 0 {
            // 追加したノードが根ノードなら二分ヒープへの追加完了
            return;
        }

        // 二分ヒープを満たすまで、追加したノードを根の方向に移動する
        loop {
            // 親ノードの位置を取得
            parent = (add - 1) / 2;

            if a[parent] < a[add] {
                // 親と子で大小関係が逆ならデータを交換
                let exchange = a[parent];
                a[parent] = a[add];
                a[add] = exchange;

                // 追加ノードは親ノードの位置に移動
                add = parent;
                if add == 0 {
                    // 追加ノードが根ノードまで移動したら二分ヒープへの追加完了
                    break;
                }else{
                    // 大小関係OKなら二分ヒープへの追加完了
                    break;
                }
            }
        }
        // 二分ヒープのデータ数が増えたのでsizeも1増やす
        size = size + 1;
    }
    // 二分ヒープの根ノードを１つずつ取り出す
    size = NUM;
    while size > 0 {
        // サイズsizeの二分ヒープからデータを１つ取り出す
        let mut left;   // 左の子ノードの位置
        let mut right;  // 右の子ノードの位置
        let mut large;  // データが大きい方の子ノードの位置
        
        // 根ノードをヒープ外に追い出す
        // 一時的に木の末端のノードを根ノードに設定する
        let mut exchange = a[0];
        a[0] = a[size - 1];
        a[size - 1] = exchange;

        // 二分ヒープのサイズを1減らす
        // これによりもともとの根ノードが「ソート済みのデータ」の先頭に移動することになる
        size = size - 1;

        // 根ノードから子ノードとの大小関係を確認していく
        let mut parent = 0;

        // 二分ヒープを満たすまで、根ノードを葉の方向に移動する
        loop {
            // 子ノードの位置を取得
            left = parent * 2 + 1;
            right = parent * 2 + 2;

            // 子ノードの大きい値を持つ方の位置を取得
            if left < size && right < size {
                // 左右両方の子ノードが存在する場合は比較して確認
                if a[left] < a[right] {
                    large = right;
                }else{
                    large = left;
                }
            }else if left < size {
                // 左の子ノードしか存在しない場合は左の子ノードを大きい値を持つとみなす
                large = left;
            }else{
                // 両ノードがヒープ内に存在しない場合は終了
                // （右子ノードしか存在しない場合はあり得ない）
                break;
            }

            if a[large] <= a[parent] {
                // すでに親子の大小関係がOKなので交換不要
                break;
            }

            // 親と子で大小関係が逆ならデータを交換する
            exchange = a[large];
            a[large] = a[parent];
            a[parent] = exchange;

            // 根ノードはデータを交換した子ノードの位置に移動する
            parent = large;
        }

        size = size - 1;
    }

    // ソート結果の表示
    println!("heap sort!");
    for i in 0..NUM {
        println!("a[{}]:{}", i, a[i]);
    }
}
