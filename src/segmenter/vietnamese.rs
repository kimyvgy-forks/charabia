use crate::segmenter::Segmenter;

use coccoc_tokenizer::word_tokenize;

/// Vietnamese specialized [`Segmenter`].
///
/// This Segmenter uses coccoc_tokenizer to segment the provided text.
/// coccoc_tokenizer: https://github.com/coccoc/coccoc-tokenizer
pub struct VietnameseSegmenter;

impl Segmenter for VietnameseSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let segment_iterator = Vec::new();

        // word_tokenize(text, None);

        println!("Hello world: Vietnamese segment: {}", to_segment);

        Box::new(segment_iterator.into_iter())
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "Viblo Mayfest - Lễ hội chia sẻ kiến thức về công nghệ thông tin được tổ chức thường niên bởi Viblo từ năm 2020.";
    const SEGMENTED: &[&str] = &[];
    const TOKENIZED: &[&str] = SEGMENTED;

    test_segmenter!(VietnameseSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Vietnamese, Language::Vie);
}