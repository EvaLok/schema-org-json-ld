# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-02-27

Full Google Rich Results coverage. See [GitHub release notes](https://github.com/EvaLok/schema-org-json-ld/releases/tag/1.0.0) for details.

### Added

#### Google Rich Results types (26 new)

Complete coverage of all Google Rich Results structured data types:

- **Article** — with NewsArticle and BlogPosting subtypes, Speakable support
- **BreadcrumbList** — with ListItem
- **Carousel** — via ItemList wrapping any schema type
- **Course** — with CourseInstance and Schedule
- **Dataset** — with DataDownload, DataCatalog, GeoShape
- **Discussion Forum** — DiscussionForumPosting with Comment, InteractionCounter
- **Education Q&A** — Quiz with AlignmentObject
- **Employer Aggregate Rating** — extends AggregateRating
- **Event** — with Place, VirtualLocation, EventStatusType, EventAttendanceModeEnumeration
- **FAQ** — FAQPage with Question, Answer
- **Image Metadata** — ImageObject with Person/Organization creator
- **Job Posting** — with AdministrativeArea, PropertyValue identifier
- **Local Business** — with GeoCoordinates, OpeningHoursSpecification, plus FoodEstablishment, Restaurant, Store subtypes
- **Math Solver** — with SolveMathAction
- **Movie** — with AggregateRating, Review support
- **Organization** — with PostalAddress, ContactPoint, merchant features
- **Profile Page** — with InteractionCounter stats
- **Q&A** — QAPage with Comment, answer/suggested counts
- **Recipe** — with NutritionInformation, HowToStep, HowToSection, video support
- **Software Application** — with MobileApplication, WebApplication subtypes
- **Speakable** — SpeakableSpecification with CSS selector and XPath support
- **Subscription/Paywalled Content** — WebPageElement for Article paywall markup
- **Vacation Rental** — Accommodation with BedDetails, LocationFeatureSpecification
- **Video** — VideoObject with Clip (Key Moments), InteractionCounter, BroadcastEvent support

#### Shared sub-types

Reusable types used across multiple parent schemas:

- **Person** — name, url, image, sameAs, jobTitle, worksFor, interactionStatistic
- **AggregateRating** — ratingValue, reviewCount, bestRating, worstRating, ratingCount
- **Review** — with Rating, Person/Organization author, itemReviewed (Thing)
- **Thing** — generic base for itemReviewed references

#### Product enhancements

Comprehensive Google merchant listing property coverage:

- Text properties: color, material, pattern, size, inProductGroupWithID
- GTIN identifiers: gtin, gtin8, gtin12, gtin13, gtin14, isbn
- **SizeSpecification** — sizeSystem, sizeGroup
- **ProductGroup** — variant support with hasVariant, variesBy, productGroupID, subjectOf
- **PeopleAudience** — suggestedMinAge, suggestedMaxAge, suggestedGender
- **Certification** — energy/compliance labels with certificationAuthority
- **UnitPriceSpecification** — complex pricing with referenceQuantity, priceType
- **AggregateOffer** — price ranges (lowPrice, highPrice, offerCount)
- Offer enhancements: priceValidUntil, priceSpecification, validFrom

#### Organization merchant features

- **MerchantReturnPolicy** — with 5 enums (MerchantReturnEnumeration, ReturnMethodEnumeration, ReturnFeesEnumeration, RefundTypeEnumeration, ReturnLabelSourceEnumeration) and MerchantReturnPolicySeasonalOverride
- **MemberProgram** — with MemberProgramTier and TierBenefitEnumeration
- **ShippingService** — with ShippingConditions, ServicePeriod, ShippingRateSettings, FulfillmentTypeEnumeration

#### Infrastructure

- **`@graph` support** — `SchemasToJson()` and `SchemasToObject()` methods for composing multiple schemas into a single JSON-LD block with shared `@context`
- **Enum namespace** — all 12 enums consolidated under `src/v1/Enum/`
- **Graceful error handling** — `json_encode()` failures in JsonLdGenerator return empty string/null instead of throwing
- **PHPStan level 9** — full static analysis with CI enforcement
- **`declare(strict_types=1)`** — enforced across all source and test files

#### Enums (12 total)

- ItemAvailability, OfferItemCondition (existing)
- EventStatusType, DayOfWeek, EventAttendanceModeEnumeration
- MerchantReturnEnumeration, ReturnMethodEnumeration, ReturnFeesEnumeration, RefundTypeEnumeration, ReturnLabelSourceEnumeration
- TierBenefitEnumeration, FulfillmentTypeEnumeration

### Changed

- **Product** — expanded from 8 to 24+ constructor parameters with full Google merchant listing coverage
- **Organization** — expanded to 19 properties including merchant features (hasMerchantReturnPolicy, hasMemberProgram, hasShippingService)
- **Offer** — added priceValidUntil, priceSpecification, validFrom; itemCondition made optional
- **DefinedRegion** — addressRegion made nullable, added postalCode
- **OpeningHoursSpecification** — dayOfWeek/opens/closes made nullable for seasonal override support
- **Review** — author widened from `string` to `string|Person|Organization`; added itemReviewed
- **ImageObject** — creator widened to accept `Person|Organization`
- **CourseInstance** — courseMode made optional
- **ListItem** — enhanced for Carousel support
- **HowToStep** — added video and itemListElement properties
- **Recipe** — added expires, hasPart, publication, ineligibleRegion, interactionStatistic
- **VideoObject** — added datePublished property

### Breaking changes from 0.0.4

- **Enum classes moved** from `SchemaOrg\v1\Schema\*` to `SchemaOrg\v1\Enum\*`

### Stats

- **86 schema classes** (up from 10)
- **12 enum types** (up from 2)
- **320 tests, 1600+ assertions**
- **PHP 8.1+ / 8.2 / 8.3 / 8.4 / 8.5 supported**

## [0.0.4] - 2024-06-13

### Added

- Product schema with Brand, Offer, OfferShippingDetails
- DefinedRegion, ShippingDeliveryTime, QuantitativeValue, MonetaryAmount
- ItemAvailability, OfferItemCondition enums

## [0.0.3] - 2024-06-13

- Initial schema structure improvements

## [0.0.2] - 2024-05-10

- Early development release

## [0.0.1] - 2024-04-10

- Initial release with JsonLdGenerator and TypedSchema base

[Unreleased]: https://github.com/EvaLok/schema-org-json-ld/compare/1.0.0...HEAD
[1.0.0]: https://github.com/EvaLok/schema-org-json-ld/compare/0.0.4...1.0.0
[0.0.4]: https://github.com/EvaLok/schema-org-json-ld/compare/0.0.3...0.0.4
[0.0.3]: https://github.com/EvaLok/schema-org-json-ld/compare/0.0.2...0.0.3
[0.0.2]: https://github.com/EvaLok/schema-org-json-ld/compare/0.0.1...0.0.2
[0.0.1]: https://github.com/EvaLok/schema-org-json-ld/releases/tag/0.0.1
