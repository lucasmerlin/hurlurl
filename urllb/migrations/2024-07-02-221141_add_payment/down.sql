-- This file should undo anything in `up.sql`

alter table links
    drop column stripe_session_id;

alter table links
    drop column payment_status;