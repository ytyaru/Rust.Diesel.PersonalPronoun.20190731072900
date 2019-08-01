-- 既存のテーブルをリネーム
alter table FirstPersons rename to tmp_FirstPersons;
-- 新しいテーブルを作成（元々のテーブル名と同じ名前で）
create table FirstPersons(
    id integer not null primary key,
    value text not null,  -- 代表的な表記
    ruby text not null default '',   -- ふりがな、ルビ
    comment text not null default '' -- 補足
);
-- レコードを全て移す
insert into FirstPersons(id, value, ruby, comment) select id, value, ruby, comment from tmp_FirstPersons;
-- 元のテーブルを削除
drop table tmp_FirstPersons;
