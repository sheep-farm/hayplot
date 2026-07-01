# Comparação: hayplot vs ggplot2

## Aderência Atual: ~60-65%

Atualizado: v1.3.0 (jul 2026)

## ✅ Implementado (Core + Básico + Crítico)

### Core Functions (100%)
- `ggplot()` → `hayplot()` ✓ - aceita múltiplas séries x: `{"x": "col1,col2", "y": "col_y"}` (separado por vírgulas) - NOVO v1.3.0
- `aes()` → via dict ✓
- `labs()` → `labs()` ✓
- `ggsave()` → `save_svg()`, `save_png()` ✓

### Geometries Básicas (85% - 12/14 mais usadas)
- ✓ point, line, bar, histogram, boxplot, area
- ✓ hline, vline, abline (reference lines)
- ✓ step (step charts)
- ✓ smooth (linear regression with CI) - NOVO v1.2.0
- ✓ text (annotations) - NOVO v1.2.0
- ✗ `geom_density` (comum ~20% dos plots)
- ✗ `geom_col` (horizontal) - parcial (bar cobre vertical, coord_flip resolve horizontal)

### Scales (70%)
- ✓ `scale_x/y_log10`
- ✓ `scale_x/y_continuous` com limits - NOVO v1.2.0
- ✗ breaks/labels (estrutura armazenada, não implementado em render)
- ✓ Paleta automática para múltiplas séries (color="auto") - NOVO v1.3.0
- ✗ `scale_color/fill_*` paletas completas
- ✗ `scale_size`

### Coordinates (50%)
- ✓ `coord_flip` - NOVO v1.2.0 (não suportado com múltiplas séries)
- ✗ `coord_fixed`
- ✗ `coord_polar`

### Themes (50%)
- ✓ `set_background_color`, `set_grid`
- ✓ `theme_element_text` (font family, size, color) - NOVO v1.2.0
- ✗ Sistema `theme_*` completo
- ✗ element_rect, element_line

### Stats (20%)
- ✓ `stat_smooth` implementado via `geom_smooth` (OLS regression)
- ✗ `stat_summary`
- ✗ `stat_density`
- ✗ `stat_boxplot`
- ✗ `stat_count`
- ✗ `stat_ecdf`
- ✗ `stat_function`

### Position Adjustments (0%)
- ✗ `position_dodge`
- ✗ `position_stack`
- ✗ `position_fill`
- ✗ `position_jitter`

### Annotations (100%)
- ✓ `geom_text` ✓
- ✗ `geom_label` (similar to text com background)

### Faceting (0%)
- ✗ `facet_wrap/grid` funcional (descontinuado por bug, substituído por filter_data)

## 🎯 Múltiplas Séries (NOVO v1.3.0)

Suporte generalista para múltiplas séries no eixo x, habilitando visualizações como DiD:

```text
// DiD: tratamento vs controle ao longo do tempo
let plot = gg :: hayplot(df, {"x": "y_control,y_treated", "y": "period"})
    |> gg :: geom_line("auto", 2.0)
    |> gg :: geom_point("auto", 3.0)
```

**Implementação:**
- `aes` aceita string separada por vírgulas: `{"x": "col1,col2,...", "y": "col_y"}`
- `geom_point("auto", ...)` e `geom_line("auto", ...)` usam paleta automática
- Paleta de 8 cores: steel blue, crimson, forest green, dark orange, purple, deep sky blue, hot pink, lime green
- Geometries point/line: suporte completo a múltiplas séries
- Outras geometries: usam primeira série (simplificado)
- coord_flip: não suportado com múltiplas séries

**Diferença vs ggplot2:**
- ggplot2 usa `aes(color=group)` com formato long
- hayplot usa `aes(x="col1,col2")` com formato wide (separado por vírgulas)
- Ambos produzem visualização idêntica (múltiplas séries coloridas)

## ❌ Falta para 80%+ Aderência

### Top 5 Mais Importantes (Restantes)
1. **`geom_density`** - Distribuições (comum ~20% dos plots)
2. **`scale_color/fill_*`** - Paletas de cores (usado em ~40% dos plots)
3. **`position_*`** - Stacking/jittering (usado em ~25% dos plots)
4. **`theme_*` completo** - Customização visual avançada (usado em ~50% dos plots)
5. **Faceting funcional** - Small multiples (usado em ~20% dos plots)

### Top 10 Mais Importantes (Restantes)
6. `coord_fixed` (aspect ratio fixo)
7. Paletas de cores (scale_brewer, scale_viridis)
8. Geometrias avançadas (geom_ribbon, geom_errorbar, geom_polygon)
9. Stats system completo (stat_summary, stat_density, stat_ecdf)
10. Coordinate systems complexos (coord_polar)

## 📊 Estimativa de Esforço

### Para 80% Aderência (Core + Comum)
- `geom_density` - Médio
- `scale_color/fill_*` (paletas básicas) - Baixo
- `position_dodge/stack` - Médio
- `theme_*` básico adicional (element_rect, element_line) - Baixo
- Faceting funcional (sem bugs) - Alto (requer refatoração)

**Estimativa:** 3-4 semanas de trabalho focado

### Para 95% Aderência (Quase completo)
- Todo acima +
- Stats system completo
- Paletas de cores avançadas
- Geometrias avançadas
- Coordinate systems (coord_fixed, coord_polar)

**Estimativa:** 8-10 semanas de trabalho focado

### Para 100% Aderência (Completo)
- Todo acima +
- Extensibilidade (criar geoms/stats customizados)
- Sistema de guides completo
- Suporte a dates/POSIXct
- Suporte a SF objects (spatial)

**Estimativa:** 14-18 semanas de trabalho focado + arquitetura extensível

## 🎯 Progresso Recente (v1.2.0)

### Implementado (jul 2026)
- `geom_smooth`: Regressão linear OLS com bandas de confiança (95% CI)
- `geom_text`: Anotações textuais em coordenadas específicas
- `scale_x/y_continuous`: Controle de limits (breaks/labels pendente implementação)
- `coord_flip`: Inversão de eixos (swap x/y)
- `theme_element_text`: Customização de fonte (family, size, color)

### Impacto na Aderência
- Geometrias: 70% → 85% (+15%)
- Scales: 40% → 70% (+30%)
- Coordinates: 0% → 50% (+50%)
- Themes: 20% → 50% (+30%)
- **Total estimado: 35-40% → 60-65% (+25%)**

## 🎯 Recomendação

**Foco imediato (para chegar a 80%):**
1. `geom_density` (distribuições - muito comum)
2. `scale_color/fill_*` (paletas básicas - muito usado)
3. `position_dodge/stack` (barras agrupadas - comum)
4. `theme_*` adicional (element_rect, element_line)
5. Faceting funcional (requer refatoração complexa)

**Isso cobriria ~80% dos casos de uso comuns.**
