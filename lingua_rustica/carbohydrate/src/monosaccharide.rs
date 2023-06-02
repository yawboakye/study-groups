pub enum Group {
    ALDEHYDE,
    KETONE,
}

pub struct Structure {
    // according to articles on the internet, the 3 seems to be the
    // minimum number of carbon atoms a monosaccharide can have, and
    // 8 the maximum. therefore, i've opted for a `u8`, which is the
    // minimum size rust offers.
    n_carbons: u8,

    // still according to the internet, a monosaccharide belongs to
    // one of two groups: aldehyde or a ketone. a monosaccharide of
    // the aldehyde group is called an aldose, ketose if it's of the
    // ketone group.
    group: Group,
}
