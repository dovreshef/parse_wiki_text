use crate::{
    state::State,
    Configuration,
    Node,
};

pub fn parse_magic_word(state: &mut State, configuration: &Configuration) {
    if let Ok((match_length, _)) = configuration
        .magic_words
        .find(&state.wiki_text[state.scan_position + 2..])
    {
        let end_position = match_length + state.scan_position + 2;
        if state.get_byte(end_position) == Some(b'_')
            && state.get_byte(end_position + 1) == Some(b'_')
        {
            let scan_position = state.scan_position;
            state.flush(scan_position);
            state.flushed_position = end_position + 2;
            state.nodes.push(Node::MagicWord {
                end: state.flushed_position,
                start: state.scan_position,
            });
            state.scan_position = state.flushed_position;
            return;
        }
    }
    state.scan_position += 1;
}
