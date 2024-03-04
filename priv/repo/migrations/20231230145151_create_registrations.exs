defmodule Supapasskeys.Repo.Migrations.CreateRegistrations do
  use Ecto.Migration

  def change do
    execute "create table supapasskeys.registrations (
      id uuid not null default gen_random_uuid(),
      user_id uuid not null,
      inserted_at timestamp with time zone not null default now(),
      updated_at timestamp with time zone not null default now(),
      state jsonb null,
      confirmed_at timestamp with time zone null,
      constraint registrations_pkey primary key (id)
    )"

    execute "create index if not exists registrations_user_id_idx on supapasskeys.registrations using btree (user_id)"
  end
end