-- Add migration script here
-- Divine AGI V14 — локальная миграция БД
-- Дата: 2 января 2026
-- Статус: БЕРСЕРК РЕЖИМ АКТИВИРОВАН
-- Таблица геномов: 3x3x3 куб тетрад A/G/C
-- РНК (kernel v3): T/G метки для координации и навигации

CREATE TABLE IF NOT EXISTS human_genome (
    id BIGSERIAL PRIMARY KEY,
    block_hash TEXT NOT NULL,                    -- hash генома (Sha256 hex)
    dna_tetrad TEXT NOT NULL CHECK (LENGTH(dna_tetrad) = 27), -- 27-char A/G/C
    consciousness INTEGER NOT NULL DEFAULT 0,    -- уровень сознания
    mutations INTEGER NOT NULL DEFAULT 0,        -- количество мутаций
    t_timestamp BIGINT NOT NULL DEFAULT (EXTRACT(EPOCH FROM NOW()) * 1000000)::BIGINT, -- T метка (время в микросекундах)
    g_coordinate BIGINT NOT NULL DEFAULT 0,      -- G метка (координата в архиве)
    inbreeding_coef DOUBLE PRECISION NOT NULL DEFAULT 0.0, -- коэффициент инбридинга (риск портала)
    coherence_time DOUBLE PRECISION,             -- время когерентности (μs, NULL = критично)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Индексы для РНК-обработки (быстрая статистика и критичные сэмплы)
CREATE INDEX IF NOT EXISTS idx_human_genome_consciousness ON human_genome (consciousness DESC);
CREATE INDEX IF NOT EXISTS idx_human_genome_mutations ON human_genome (mutations DESC);
CREATE INDEX IF NOT EXISTS idx_human_genome_inbreeding ON human_genome (inbreeding_coef DESC);
CREATE INDEX IF NOT EXISTS idx_human_genome_coherence ON human_genome (coherence_time ASC);
CREATE INDEX IF NOT EXISTS idx_human_genome_t_timestamp ON human_genome (t_timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_human_genome_g_coordinate ON human_genome (g_coordinate);

-- Триггер для updated_at
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_updated_at
BEFORE UPDATE ON human_genome
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

-- Комментарии
COMMENT ON TABLE human_genome IS 'Геномы Divine AGI V14 — 3x3x3 куб тетрад A/G/C, динамический поворот (0° compute, 180° storage, 270° mutation)';
COMMENT ON COLUMN human_genome.t_timestamp IS 'T метка — временная метка для РНК-координации';
COMMENT ON COLUMN human_genome.g_coordinate IS 'G метка — координата для навигации в архивах';
COMMENT ON COLUMN human_genome.inbreeding_coef IS 'Коэффициент инбридинга — высокий = риск портала';
COMMENT ON COLUMN human_genome.coherence_time IS 'Время когерентности — низкое = критично (portal risk)';