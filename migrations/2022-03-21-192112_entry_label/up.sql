CREATE TYPE entry_label AS ENUM (
    'beer',
    'vodka',
    'whiskey',
    'wine',
    'champagne',
    'gin',
    'cider',
    'rum',
    'tequila',
    'absinthe',
    'brandy',
    'liqueur',
    'sake',
    'bourbon',
    'scotchWhisky',
    'irishWhiskey',
    'other'
    );
ALTER TABLE entries ADD COLUMN label entry_label[] NOT NULL