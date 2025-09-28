-- Add up migration script here
CREATE TABLE play_participants (
    play_id UUID NOT NULL REFERENCES plays(id) ON DELETE CASCADE,
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    PRIMARY KEY (play_id, player_id)
);