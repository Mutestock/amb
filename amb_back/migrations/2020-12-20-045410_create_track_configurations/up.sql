-- Your SQL goes here

CREATE TABLE track_configurations (
    id SERIAL PRIMARY KEY NOT NULL,
    track_id INTEGER REFERENCES tracks(id),
    playlist_id INTEGER REFERENCES playlists(id),
    is_looping BOOLEAN NOT NULL,
    is_volume_random BOOLEAN NOT NULL,
    is_internal_random BOOLEAN NOT NULL,
    volume_min FLOAT NOT NULL,
    volume_max FLOAT NOT NULL,
    interval_min INTEGER NOT NULL,
    interval_max INTEGER NOT NULL,
    pan_x INTEGER NOT NULL,
    pan_y INTEGER NOT NULL,
    pan_z INTEGER NOT NULL,
    pitch_shift INTEGER NOT NULL,
    reverb INTEGER NOT NULL,
    compressor_threshold INTEGER NOT NULL,
    compressor_gain_reduction INTEGER NOT NULL
)