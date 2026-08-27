-- Ejecutar en el SQL Editor de tu proyecto Supabase
-- Dashboard > SQL Editor > New query > pegar esto > Run

create table if not exists waitlist (
  id uuid default gen_random_uuid() primary key,
  email text not null unique,
  lang text default 'es',
  source text default 'landing',
  created_at timestamptz default now()
);

-- Permitir INSERT anónimo (el anon key es público, esto es seguro)
alter table waitlist enable row level security;

create policy "Allow anonymous insert"
  on waitlist for insert
  to anon
  with check (true);

-- Bloquear lectura/update/delete desde el frontend
create policy "Block anonymous read"
  on waitlist for select
  to anon
  using (false);
