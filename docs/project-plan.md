# Small Business Pages Platform: Project Plan

## 1. Overview

Many small businesses run entirely through WhatsApp. They have no web presence, so there is nowhere to send a new client to see what they offer. This project is a multi-tenant website platform where each business gets its own public page, which they manage themselves.

Each business gets a unique link (e.g. `yourplatform.com/b/joes-bakery`) to share with clients. Clients only ever see that one business, never the others.

## 2. Goals and Non-Goals

**Goals**
- Give each business a simple, good-looking public page in minutes
- Let business owners edit their own content without help
- Let us (the admins) onboard, manage, and suspend businesses
- Keep every business's data and page isolated from the others
- Make it easy for clients to contact the business, especially via WhatsApp

**Non-goals (for v1)**
- Online payments or checkout
- Full e-commerce or inventory
- Custom code per business
- Replacing WhatsApp (we complement it)

## 3. Users and Roles

| Role | Description | Access |
|---|---|---|
| Client / visitor | Someone who opens a business link | Public page only, no login |
| Business owner | Signed-up business | Edit their own page only |
| Admin (us) | Platform operators | Create/suspend businesses, view all, support |

## 4. Core Features

### Public business page
- Business name, logo, cover image, description
- Services or product list (name, description, price, photo)
- Opening hours and location (with map link)
- Contact buttons: **"Chat on WhatsApp"** (a `wa.me` link with an optional prefilled message), phone, email
- Photo gallery
- Social links
- Mobile-first design (almost all visitors will arrive from a phone)

### Business dashboard (login required)
- Edit profile, services, hours, gallery
- Choose a theme/colour from a few presets
- Preview before publishing
- Publish / unpublish toggle
- Basic analytics (page views, WhatsApp button clicks)

### Admin panel
- Create, invite, edit, suspend, and delete businesses
- View all businesses and their status
- Manage slugs (reserved names, conflicts)
- Basic audit log of who changed what

## 5. Architecture

```
Browser (React SPA)  <->  Rust API (Axum)  <->  PostgreSQL
                              |
                              +--> Object storage (images)
                              +--> Google OIDC (auth)
```

### Frontend: React
- Vite + React + TypeScript
- Router with three areas: public pages, business dashboard, admin
- Tailwind CSS (or similar) for fast, consistent styling
- **SEO consideration:** a client-rendered SPA is weak for search engines and link previews. Since links will be shared on WhatsApp, **link previews (Open Graph tags) matter a lot**. Options:
  - Use a framework with server-side rendering (Next.js or Remix), or
  - Have the Rust backend serve pre-rendered HTML with meta tags for public pages, while the dashboard stays a SPA
  - Decide this early; it is the biggest frontend architecture choice

### Backend: Rust
- **Axum** web framework (Tokio-based, well maintained)
- **SQLx** for async, compile-time-checked Postgres queries
- `openidconnect` or `oauth2` crate for Google login
- `tower-sessions` or signed/encrypted cookies for sessions
- `serde` for JSON, `tracing` for logs, `validator` for input checks
- `tower-http` for CORS, compression, rate limiting

### Database: PostgreSQL
Suggested starting schema:

- `businesses` (id, slug, name, description, logo_url, status, theme, created_at)
- `business_contacts` (business_id, whatsapp_number, phone, email, address, maps_url)
- `business_hours` (business_id, weekday, open_time, close_time)
- `services` (id, business_id, name, description, price, image_url, sort_order)
- `gallery_images` (id, business_id, url, sort_order)
- `users` (id, google_sub, email, name, role, created_at)
- `business_members` (user_id, business_id, role): links owners to businesses
- `invitations` (id, business_id, email, token, expires_at, used_at)
- `page_events` (id, business_id, type, created_at): for simple analytics
- `audit_log` (id, user_id, action, target, created_at)

### Multi-tenancy
- **Shared database, shared schema**, with a `business_id` on every tenant-owned row
- Every query on the dashboard side must be scoped by the authenticated user's business. Enforce this in one place (a repository layer or middleware), not ad hoc
- Optional extra safety: Postgres Row-Level Security
- **Public URL strategy:**
  - Path-based (`/b/slug`): simplest, one domain, one TLS cert. **Recommended for v1**
  - Subdomain-based (`slug.yourplatform.com`): nicer branding, needs wildcard DNS + wildcard cert
  - Custom domains per business: a possible paid upgrade later, needs automated certificate issuance

### File and image storage
- Do not store images in Postgres
- Use S3-compatible storage (Cloudflare R2, Backblaze B2, S3) with a CDN in front
- Resize and compress on upload (WhatsApp-sourced photos are often huge)
- Enforce size and type limits server-side

## 6. Authentication and Authorisation

**Plan:** Google as OpenID Connect provider (Authorization Code flow with PKCE).

**Flow**
1. User clicks "Sign in with Google"
2. Backend redirects to Google with `state`, `nonce`, and a PKCE challenge
3. Google redirects back with a code
4. Backend exchanges the code, validates the ID token (signature, `aud`, `iss`, `exp`, `nonce`)
5. Backend looks up the user by Google `sub` (not by email alone) and starts a session
6. Session stored in an `HttpOnly`, `Secure`, `SameSite=Lax` cookie

**Authorisation**
- Roles: `admin`, `owner`
- Admins are defined by an allowlist (config or DB flag), never self-assigned
- Business owners can only sign in if **we invited them** (invite link tied to their business). No open self-registration, at least in v1
- Every API route checks role *and* business ownership

**Things to think about**
- Some small business owners may not have, or want to use, a Google account. Consider a fallback later (email magic link, or phone OTP via WhatsApp)
- Matching on `sub` is safer than on email, since emails can change
- Plan how to handle a business owner who loses access to their Google account (admin recovery process)
- CSRF protection on all state-changing requests (SameSite cookie + CSRF token or origin checks)

## 7. API Sketch

**Public**
- `GET /api/public/businesses/:slug`: business page data
- `POST /api/public/businesses/:slug/events`: record a view or click

**Auth**
- `GET /auth/google/login`, `GET /auth/google/callback`, `POST /auth/logout`
- `GET /api/me`

**Business dashboard**
- `GET/PUT /api/my/business`
- `CRUD /api/my/business/services`
- `POST /api/my/business/images` (upload), `DELETE /api/my/business/images/:id`
- `POST /api/my/business/publish`

**Admin**
- `GET/POST /api/admin/businesses`
- `PATCH /api/admin/businesses/:id` (suspend, edit slug)
- `POST /api/admin/businesses/:id/invite`
- `GET /api/admin/audit-log`

## 8. Infrastructure and Deployment

- **Hosting:** a small VPS (Hetzner, DigitalOcean) or a platform like Fly.io/Railway. A single Rust binary plus Postgres is cheap to run
- **Containerisation:** Docker for consistent builds
- **Reverse proxy / TLS:** Caddy or Nginx, or Cloudflare in front
- **CI/CD:** GitHub Actions (lint, test, build, deploy)
- **Environments:** local, staging, production
- **Backups:** automated daily Postgres backups, tested restores
- **Monitoring:** uptime check, error tracking (e.g. Sentry), structured logs
- **Secrets:** environment variables or a secrets manager; never in the repo

## 9. Security Checklist

- Validate and sanitise all input; escape output (guard against XSS in business-supplied text)
- Reserved slugs (`admin`, `api`, `login`, etc.) and slug format rules
- Rate limit login, uploads, and public endpoints
- Strict upload validation (type, size, image re-encoding)
- Content Security Policy headers
- Dependency auditing (`cargo audit`, `npm audit`)
- Least-privilege DB user
- Never trust the client for `business_id`; always derive it from the session

## 10. Legal and Compliance

- Privacy policy and terms of service
- Data protection law for your region (e.g. POPIA in South Africa, GDPR in the EU/UK): you will store business-owner personal data and possibly visitor analytics
- Cookie notice if analytics or non-essential cookies are used (cookie-free analytics avoids most of this)
- Clear policy on who owns the content and what happens when a business leaves
- Content moderation and a takedown process for abuse

## 11. Business Model (to discuss)

- Free for early adopters / pilot businesses
- Monthly subscription per business
- Paid tiers: custom domain, more photos, analytics, remove platform branding
- One-time setup fee where you build the page for them

## 12. Roadmap

**Phase 0: Discovery (1 week)**
- Talk to the two known businesses; list what they actually need on a page
- Collect examples they like
- Decide: SSR vs SPA, path vs subdomain

**Phase 1: MVP (4 to 8 weeks)**
- Rust API skeleton, Postgres, migrations
- Google login + invitation flow
- Business dashboard: profile, services, hours, images
- Public page with one clean, responsive template
- Admin panel: create/invite/suspend
- Deploy to staging, then onboard the two pilot businesses

**Phase 2: Polish**
- Open Graph previews, SEO basics
- Basic analytics
- A few themes
- QR code generator for each business link (useful for print and storefronts)

**Phase 3: Growth**
- Custom domains
- Subscription billing
- Extra login methods
- Optional WhatsApp integrations (e.g. WhatsApp Business catalogue sync)

## 13. Risks

| Risk | Mitigation |
|---|---|
| Tenant data leakage between businesses | Central scoping layer, tests for it, optional RLS |
| Owners find the dashboard too complex | Keep v1 tiny; test with the two pilot businesses |
| Google-only login excludes some owners | Plan a fallback login early |
| Poor link previews on WhatsApp | Server-rendered meta tags from day one |
| Scope creep | Hold the non-goals list; ship the MVP first |
| Image storage costs / abuse | Size limits, compression, per-business quotas |
| Two-person bus factor | Document decisions; keep the stack small |

## 14. Open Questions

1. Path-based URLs or subdomains for v1?
2. SSR framework, or Rust-rendered public pages with a React dashboard?
3. Will we build pages *for* businesses, or are they fully self-service?
4. What is the pricing and who handles billing and support?
5. Which languages must the pages support (single vs multi-language)?
6. Do businesses need a booking or enquiry form, or is WhatsApp contact enough?
7. Who owns the project legally (partnership, company), and how is revenue split?

## 15. Immediate Next Steps

- [ ] Interview the two businesses and write down their top 5 needs
- [ ] Decide on rendering approach and URL structure
- [ ] Set up two separate repos (one for the React frontend, one for the Rust backend), each with its own CI and Docker setup
- [ ] Register a domain name
- [ ] Create a Google Cloud project and OAuth credentials
- [ ] Sketch wireframes of the public page and dashboard
- [ ] Define the MVP scope in writing and agree on it with your friend
