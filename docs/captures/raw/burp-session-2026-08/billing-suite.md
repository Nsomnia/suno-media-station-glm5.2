# Billing Suite (T1 capture 2026-08-25)

All `GET` unless noted, on `https://studio-api-prod.suno.com`. Bearer +
Browser-Token + Device-Id on all.

## `GET /api/billing/info/`

- Primary subscription/credits read. Response → 200 (~22 KB), top-level keys:
```json
{
  "subscription_platform": "stripe",
  "is_active": true,
  "is_past_due": false,
  "credits": 5,
  "subscription_type": true,
  "subscription_anchor": "2026-08-13T06:12:58Z",
  "subscription_id": "<redacted-subscription-id>",
  "renews_on": "2026-09-13T06:12:58Z",
  "period": "month",
  "monthly_usage": 1050,
  "monthly_limit": 10000,
  "credit_packs": [
    {"id": "<uuid>", "amount": 500, "price_usd": 4,
     "price_amount": "4.00000", "price_currency_code": "USD"},
    "...3 more tiers (1000/$8, 2000/$16, 4000/$30)..."
  ],
  "download_credit_packs": [],
  "plan": {
    "id": "<uuid>", "level": 30, "plan_key": "premier",
    "name": "Premier Plan", "features": "<marketing text>",
    "monthly_price_usd": 30.0, "annual_price_usd": 288.0,
    "usage_plan_features": [{"name": "v4"}, {"name": "cover"}, {"name": "get_stems"}, "...feature flags..."],
    "prices": [{"period_type": "month", "currency": "USD", "price": 30.0}, "...multi-currency..."]
  },
  "...more keys..."
}
```
- `plan.usage_plan_features[].name` is the feature-flag vocabulary also seen
  in usage-plans (`v4`, `cover`, `edit_mode`, `persona`, `commercial_rights`,
  `get_stems`, `generate_song_video`, `custom_models`, …).

## `GET /api/billing/usage-plans` — plan catalog (no auth observed beyond Browser-Token/Device-Id; no Authorization header captured)

```json
{"plans": [
  {"id": "<uuid>", "level": 0, "plan_key": "free", "name": "Free Plan",
   "features": "<text>", "monthly_price_usd": 0.0, "annual_price_usd": 0.0,
   "usage_plan_features": [{"name": "tag_upsample"}], "prices": []},
  {"...pro...", "level": 10, "monthly_price_usd": 10.0},
  "...basic / premier / pro_20250501 variants..."
]}
```
- Plan keys observed: `free`, `basic`, `pro`, `premier`, `pro_20250501`
  (grandfathered variant key appears in comparison tables).

## Supporting marketing/config reads

| Endpoint | Response shape |
|---|---|
| `GET /api/billing/usage-plan-descriptions/` | `{cta_buttons: {…}, usage_plan_descriptions: {<plan_key>: {badge, plan_description, feature_descriptions[], …}}}` |
| `GET /api/billing/usage-plan-faq/` | `{faq: [{question, answer}, …]}` |
| `GET /api/billing/usage-plan-web-table-comparison/` | `{usage_plan_web_table_comparison: {table_sections: [{name, features: [{name, free, basic, pro, premier, …}]}]}}` |
| `GET /api/billing/eligible-discounts/` | `{eligible_discounts: {}, bonus_offers: [], annual_price_overlays: []}` |
| `GET /api/billing/conversion-tracking/` | `{user: {id: "<redacted-user-id>", account_age_days: <int>}, subscription: {subscription_id: "<redacted>", subscription_anchor, plan_key, plan_name, plan_price, final_price, period}}` |

## Mutations

- **`POST /api/billing/auto-reload/nudge-check`** → 200:
  `{"show": false}` (decides whether to show the auto-reload upsell nudge).
