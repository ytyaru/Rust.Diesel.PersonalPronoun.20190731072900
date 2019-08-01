create table SecondPersons(
    id integer not null primary key,
    value text not null,             -- 代表的な表記
    ruby text not null default '',   -- ふりがな、ルビ
    comment text not null default '' -- 補足
);
