CREATE TABLE newsletter_issues
(
    newsletter_issue_id uuid PRIMARY KEY,
    title               TEXT NOT NULL,
    text_content        TEXT NOT NULL,
    html_content        TEXT NOT NULL,
    published_at        TEXT NOT NULL
);