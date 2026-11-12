# Small Business Pages Platform: Project Plan

## 1. Overview

Many small businesses run entirely through WhatsApp. They have no web presence, so there is nowhere to send a client to see what they offer, and orders and questions get buried in chat threads.

This project is a multi-tenant website platform where each business gets its own public page, which they manage themselves. Each business gets a unique link to share with clients. Clients only ever see that one business, never the others.

**First use case (MVP focus):** a friend who makes food to order and delivers it. His customers currently ask for meals over WhatsApp, with no easy way to see what he makes or what it costs. He needs a menu with prices and a way for customers to place an order that notifies him.

The platform is built around this use case, but kept generic so any business that sells a list of things (food, baked goods, crafts, services) can use it.

## 2. Goals and Non-Goals

**Goals**
- Give each business a simple, good-looking public page with a menu or catalogue
- Let customers browse items with photos and prices
- Let customers submit an order request through the page, which notifies the business
- Keep WhatsApp as the channel for questions and follow-up
- Let business owners manage their own content and orders without help
- Let us (the admins) onboard and manage businesses
- Keep each business's data and page isolated from the others

**Non-goals (for v1)**
- Online payments
- Inventory or stock tracking
- Delivery fee calculation, routing, or tracking
- Custom fields or custom layouts per business
- Customer accounts
- Replacing WhatsApp (we complement it)

## 3. Users and Roles

| Role | Description | Access |
|---|---|---|
| Customer | Someone who opens a business link and places an order request | Public page only, no login |
| Business owner | A signed-up business | Manage their own page, menu, and orders |
| Admin (us) | Platform operators | Create/suspend businesses, view all, support |

## 4. MVP Scope

### Public business page
- Business name, logo, short description
- **Menu:** a list of items, each with a photo, name, short description, and price
- Contact details and a **"Message on WhatsApp"** button for questions
- Opening hours and delivery area / notes (free text)
- Mobile-first design, since most visitors will arrive from a phone via WhatsApp

### Ordering
- Customer adds items to an order and sets quantities
- Customer fills in a short order form:
  - Name
  - Phone number (and optionally email)
  - Delivery or collection
  - Delivery address (if delivery)
  - Requested date/time
  - **A free-text notes box** for anything extra (allergies, changes, special requests)
- Customer submits the order. This is an **order request**, not a payment or a guaranteed sale
- Customer sees a confirmation screen with a reference number and a message that the business will be in touch
- The business owner is **notified immediately** when a new order arrives

### Business dashboard (login required)
- Edit business profile, contact details, and hours
- **Manage menu items:** add, edit, remove, reorder; upload a photo; set a name, description, and price; mark an item as unavailable without deleting it
- **View and manage orders:** see new orders, order contents, customer details, and notes
- Update order status (for example: New, Confirmed, Completed, Declined)
- Quick "reply on WhatsApp" link on each order that opens a chat with the customer
- Publish / unpublish the page

### Admin panel
- Create, invite, edit, and suspend businesses
- View all businesses and their status
- Manage business links (slugs) and reserved names
- Basic audit log

## 5. Key User Flows

**Customer ordering**
1. Opens the business link (shared on WhatsApp)
2. Browses the menu
3. Adds items, fills in the order form, submits
4. Sees a confirmation with a reference number

**Business owner receiving an order**
1. Gets a notification (see section 6)
2. Opens the dashboard and reviews the order
3. Contacts the customer on WhatsApp if needed, then confirms or declines
4. Marks the order as completed once delivered

**Onboarding a business**
1. Admin creates the business and sends an invite
2. Owner signs in, fills in their profile, and adds menu items (or we do the first setup for them)
3. Owner publishes the page and shares the link

## 6. Order Notifications

- **MVP:** email notification to the business owner for every new order, plus a visible "new orders" list in the dashboard
- Optional, low-effort addition: after submitting, offer the customer a button to send their order summary to the business via WhatsApp as well
- **Later:** automated WhatsApp or SMS notifications to the owner. These typically need a paid business messaging service, so they are out of scope for now
- Open question: should the customer also receive an email confirmation if they provide an email address?

## 7. Making It Generic

To keep the platform reusable beyond food:

- Use neutral terms in the code and data model: **business**, **items**, **orders**. Only the display wording changes (for example "Menu" vs "Products" vs "Services")
- Each item has the same small set of fields: photo, name, description, price, availability
- Orders are a list of items plus customer details and a free-text note
- A per-business setting controls whether ordering is enabled, and whether delivery, collection, or both are offered
- No per-business custom fields in v1. The notes box covers special requests

## 8. Page Structure and Look

Fixed structure, with a few style options, rather than a page builder:

1. Header: logo, name, tagline, WhatsApp button
2. About: short description
3. Menu / catalogue
4. Order summary and form
5. Hours, delivery area, and contact details

Owners control content, and can choose from a small number of themes and an accent colour. We control layout, so every page looks good by default.

Design priorities: mobile first, fast loading, clear prices, large appealing photos, and a very simple ordering flow.

## 9. Architecture (High Level)

- **Frontend:** React application with a public area (customer-facing), a business dashboard, and an admin area
- **Backend:** Rust API
- **Data storage:** a relational database for businesses, items, orders, and users, plus file storage for images
- **Authentication:** for business owners and admins only (see section 10). Customers do not log in
- **Repositories:** two separate repos, one for the frontend and one for the backend. The API contract between them should be documented (for example with an OpenAPI spec) so the two stay in sync

Specific libraries, frameworks, and hosting choices are deliberately left open for now.

**Things to decide early**
- How public pages are rendered. Business links will mostly be shared on WhatsApp, so **link previews** (title, image, description) need to work well, which affects whether pages are rendered on the server or in the browser
- URL structure for businesses: a path per business (for example `/b/business-name`) is simplest to start with; subdomains or custom domains can come later

### Multi-tenancy
- All businesses share one system, and every business-owned record carries its business identifier
- Business owners can only ever access their own business's data. This check should live in one central place rather than being repeated ad hoc across the code
- Public pages only ever return data for the single business in the link

## 10. Authentication and Authorisation

Login is only for business owners and admins.

**Options under consideration**
- Sign in with Google (OpenID Connect)
- Magic email link (a one-time login link sent by email)
- Possibly both, with one as the fallback

**Considerations**
- Some owners may not have, or want to use, a Google account
- Magic links remove the need for passwords and Google setup, but make email deliverability and link security our responsibility
- Whichever is chosen, identify users by an internal ID, not by email alone, so login methods can be added or changed later
- Owners can only sign in if invited. No open self-registration in v1
- Admins are defined by us, never self-assigned
- Every request must check the user's role and which business they belong to

## 11. Data Overview (Conceptual)

- **Business:** name, link (slug), description, logo, contact details, hours, theme, status, ordering settings
- **Item:** belongs to a business; name, description, photo, price, availability, sort order
- **Order:** belongs to a business; reference number, customer name, phone, optional email, delivery or collection, address, requested time, notes, status, created time
- **Order line:** an item, quantity, and the price at the time of ordering (so later price changes do not alter past orders)
- **User:** owner or admin; linked to a business
- **Invitation:** used to onboard new business owners
- **Audit log:** record of admin and owner actions

## 12. Security and Abuse Prevention

- Validate all input, and safely display business- and customer-supplied text
- The public order form is open to anyone, so it needs spam protection: rate limiting and a lightweight bot check
- Limit upload sizes and types for images, and resize them on upload
- Never trust the client to say which business it belongs to; derive it from the session
- Reserved link names (for example `admin`, `api`, `login`)
- Keep customer personal data (names, phone numbers, addresses) to what is needed, and plan how long it is kept

## 13. Legal and Compliance

- Privacy policy and terms of service
- Data protection law for your region applies, since we store customer and owner personal data
- Make clear that orders are requests that the business confirms, and that the business (not the platform) is responsible for the food, pricing, and delivery
- Process for removing content or suspending a business

## 14. Business Model (To Discuss)

- Free for the first pilot business(es)
- Later: a monthly subscription per business
- Possible paid extras: custom domain, more photos, order history and reports, removal of platform branding

## 15. Roadmap

**Phase 0: Discovery (about 1 week)**
- Sit down with the friend and list exactly what his menu, ordering, and delivery process looks like
- Collect sample photos and menu items to use for testing
- Decide page rendering approach and URL structure
- Decide the login method

**Phase 1: MVP**
- Business profile and menu management
- Public page with menu and photos
- Order request form and confirmation
- Order email notification and dashboard order list with statuses
- WhatsApp contact button and "reply on WhatsApp" link
- Owner login, admin panel, and invitations
- Deploy and onboard the first business

**Phase 2: Polish**
- Link previews and SEO basics
- A few themes and an accent colour
- Order history and simple stats
- QR code for each business link
- Customer email confirmation

**Phase 3: Growth**
- Automated WhatsApp/SMS order notifications
- Online payments
- Custom domains
- Subscription billing
- Item categories, options, and extras
- Support for more business types

## 16. Risks

| Risk | Mitigation |
|---|---|
| Orders are missed because notifications fail or are ignored | Reliable email delivery; clear "new orders" view in the dashboard; test with the real business |
| Owner finds the dashboard too complex | Keep it very small; test with the friend early; offer to do initial setup for them |
| Spam or fake orders | Rate limiting, bot check, and the owner confirms every order |
| Data leaking between businesses | Central access checks and tests for them |
| Customers expect instant confirmation or payment | Clear wording that this is an order request |
| Scope creep | Hold to the MVP and non-goals list |
| Poor WhatsApp link previews | Plan for this in the rendering decision |

## 17. Open Questions

1. How does the friend currently handle orders, timing, and delivery? Is there a cut-off time or minimum order?
2. Should orders ask for a requested date/time, or just "as soon as possible"?
3. Is delivery free, or does the owner want to add a delivery fee line?
4. Should prices be shown in a single currency only?
5. Will we set up each business for the owner at first, or is it fully self-service?
6. Which login method(s) to use?
7. Who handles support, pricing, and billing, and how is the project owned and revenue split?
8. Do customers need an email confirmation, or is on-screen confirmation plus WhatsApp enough?

## 18. Immediate Next Steps

- [ ] Walk through the friend's ordering process and collect real menu items and photos
- [ ] Decide page rendering approach and URL structure
- [ ] Decide the login method
- [ ] Sketch wireframes: public menu page, order form, owner dashboard (menu and orders)
- [ ] Set up two repositories (frontend and backend) with basic CI
- [ ] Register a domain name
- [ ] Agree the MVP scope in writing with your friend
