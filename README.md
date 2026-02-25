# schema-org-json-ld

PHP library for generating schema.org JSON-LD structured data for Google Rich Results. Covers **27 Google Rich Results types** backed by **65 schema classes**, with type-safe constructor-promoted properties and automatic serialization.

Tested & validated using the [Google Rich Results Test](https://search.google.com/test/rich-results/result?id=eeuHnX6wLe1IqxNkqd42xA).

![PHP](https://img.shields.io/badge/PHP-8.1%2B-blue)
![License](https://img.shields.io/badge/license-MIT-green)

---

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Supported Types](#supported-types)
- [Usage Examples](#usage-examples)
  - [Article](#article)
  - [Breadcrumb](#breadcrumb)
  - [Carousel](#carousel)
  - [Course](#course)
  - [Dataset](#dataset)
  - [Discussion Forum](#discussion-forum)
  - [Education Q&A](#education-qa)
  - [Employer Aggregate Rating](#employer-aggregate-rating)
  - [Event](#event)
  - [FAQ](#faq)
  - [Image Metadata](#image-metadata)
  - [Job Posting](#job-posting)
  - [Local Business](#local-business)
  - [Movie](#movie)
  - [Organization](#organization)
  - [Product](#product)
  - [Profile Page](#profile-page)
  - [Q&A](#qa)
  - [Recipe](#recipe)
  - [Review Snippet](#review-snippet)
  - [Software App](#software-app)
  - [Speakable](#speakable)
  - [Subscription / Paywalled Content](#subscription--paywalled-content)
  - [Vacation Rental](#vacation-rental)
  - [Video](#video)
- [API Reference](#api-reference)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

---

## Installation

```bash
composer require evabee/schema-org-json-ld
```

> **Note:** The vendor name `evabee` is intentional. Due to Packagist requiring excessive GitHub permissions, the default `evalok` vendor namespace is unavailable. The package is otherwise identical.

---

## Quick Start

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Article;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;

$article = new Article(
	headline: 'How to Use Schema.org JSON-LD in PHP',
	author: new Person(name: 'Jane Smith'),
	datePublished: '2026-01-15',
);

$json = JsonLdGenerator::SchemaToJson(schema: $article);

// Embed in your HTML <head>:
// <script type="application/ld+json"><?= $json ?></script>
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Article",
    "headline": "How to Use Schema.org JSON-LD in PHP",
    "author": {
        "@type": "Person",
        "name": "Jane Smith"
    },
    "datePublished": "2026-01-15"
}
```

The pattern is always the same: instantiate a schema class, pass it to `JsonLdGenerator::SchemaToJson()`, and embed the result in a `<script type="application/ld+json">` tag.

---

## Supported Types

| Google Rich Results category | Supported schema types |
| --- | --- |
| Article | `Article`, `NewsArticle`, `BlogPosting` |
| Breadcrumb | `BreadcrumbList`, `ListItem` |
| Carousel | `ItemList`, `ListItem` |
| Course | `Course`, `CourseInstance`, `Schedule` |
| Dataset | `Dataset`, `DataDownload`, `DataCatalog` |
| Discussion forum | `DiscussionForumPosting`, `Comment`, `InteractionCounter` |
| Education Q&A | `Quiz`, `Question`, `Answer`, `AlignmentObject` |
| Employer aggregate rating | `EmployerAggregateRating` |
| Event | `Event`, `Place`, `Offer`, `EventStatusType` |
| FAQ | `FAQPage`, `Question`, `Answer` |
| Image metadata | `ImageObject` |
| Job posting | `JobPosting`, `Organization`, `Place`, `MonetaryAmount`, `AdministrativeArea` |
| Local business | `LocalBusiness`, `PostalAddress`, `GeoCoordinates`, `OpeningHoursSpecification`, `AggregateRating`, `Review` |
| Movie | `Movie`, `Person`, `AggregateRating`, `Review` |
| Organization | `Organization`, `PostalAddress`, `ContactPoint` |
| Product | `Product`, `Offer`, `Brand`, `OfferShippingDetails`, `ShippingDeliveryTime`, `DefinedRegion`, `MonetaryAmount`, `QuantitativeValue` |
| Profile page | `ProfilePage`, `Person`, `Organization` |
| Q&A | `QAPage`, `Question`, `Answer` |
| Recipe | `Recipe`, `Person`, `NutritionInformation`, `HowToStep`, `AggregateRating` |
| Review snippet | `Review`, `AggregateRating`, `Rating` |
| Software app | `SoftwareApplication`, `MobileApplication`, `WebApplication` |
| Speakable | `SpeakableSpecification` (via `Article`) |
| Subscription/paywalled content | `WebPageElement` (via `Article`) |
| Vacation rental | `VacationRental`, `Accommodation`, `BedDetails`, `PostalAddress`, `AggregateRating` |
| Video | `VideoObject` |

---

## Usage Examples

### Article

Supports `Article`, `NewsArticle`, and `BlogPosting`. `NewsArticle` and `BlogPosting` extend `Article` and share the same constructor — only `@type` differs.

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Article;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;

$article = new Article(
	headline: 'How to Use Schema.org JSON-LD in PHP',
	author: new Person(name: 'Jane Smith', url: 'https://example.com/jane'),
	datePublished: '2026-01-15',
	dateModified: '2026-02-01',
	image: ['https://example.com/photos/article.jpg'],
	publisher: new Organization(
		name: 'Example Media',
		url: 'https://example.com',
		logo: 'https://example.com/logo.png',
	),
);

$json = JsonLdGenerator::SchemaToJson(schema: $article);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Article",
    "headline": "How to Use Schema.org JSON-LD in PHP",
    "author": {
        "@type": "Person",
        "name": "Jane Smith",
        "url": "https://example.com/jane"
    },
    "datePublished": "2026-01-15",
    "dateModified": "2026-02-01",
    "image": [
        "https://example.com/photos/article.jpg"
    ],
    "publisher": {
        "@type": "Organization",
        "name": "Example Media",
        "url": "https://example.com",
        "logo": "https://example.com/logo.png"
    }
}
```

Use `NewsArticle` or `BlogPosting` by substituting the class name — the constructor is identical:

```php
use EvaLok\SchemaOrgJsonLd\v1\Schema\NewsArticle;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BlogPosting;

$newsArticle = new NewsArticle(headline: 'Breaking: PHP 8.4 Released', datePublished: '2026-01-20');
$blogPost    = new BlogPosting(headline: 'My Journey Learning PHP',    datePublished: '2026-02-10');
```

---

### Breadcrumb

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BreadcrumbList;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ListItem;

$breadcrumb = new BreadcrumbList(
	itemListElement: [
		new ListItem(position: 1, name: 'Home',             item: 'https://example.com'),
		new ListItem(position: 2, name: 'Books',            item: 'https://example.com/books'),
		new ListItem(position: 3, name: 'Science & Nature', item: 'https://example.com/books/science'),
	],
);

$json = JsonLdGenerator::SchemaToJson(schema: $breadcrumb);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "BreadcrumbList",
    "itemListElement": [
        {
            "@type": "ListItem",
            "position": 1,
            "name": "Home",
            "item": "https://example.com"
        },
        {
            "@type": "ListItem",
            "position": 2,
            "name": "Books",
            "item": "https://example.com/books"
        },
        {
            "@type": "ListItem",
            "position": 3,
            "name": "Science & Nature",
            "item": "https://example.com/books/science"
        }
    ]
}
```

---

### Carousel

Use `ItemList` with `ListItem` entries pointing to the URLs of items in the carousel (articles, recipes, movies, etc.).

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemList;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ListItem;

$carousel = new ItemList(
	itemListElement: [
		new ListItem(position: 1, url: 'https://example.com/articles/1'),
		new ListItem(position: 2, url: 'https://example.com/articles/2'),
		new ListItem(position: 3, url: 'https://example.com/articles/3'),
	],
	itemListOrder: 'ItemListOrderAscending',
	numberOfItems: 3,
);

$json = JsonLdGenerator::SchemaToJson(schema: $carousel);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "ItemList",
    "itemListElement": [
        {
            "@type": "ListItem",
            "position": 1,
            "url": "https://example.com/articles/1"
        },
        {
            "@type": "ListItem",
            "position": 2,
            "url": "https://example.com/articles/2"
        },
        {
            "@type": "ListItem",
            "position": 3,
            "url": "https://example.com/articles/3"
        }
    ],
    "itemListOrder": "ItemListOrderAscending",
    "numberOfItems": 3
}
```

---

### Course

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Course;
use EvaLok\SchemaOrgJsonLd\v1\Schema\CourseInstance;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Schedule;

$course = new Course(
	name: 'Advanced PHP Development',
	description: 'Master PHP 8.x features, design patterns, and best practices.',
	provider: new Organization(name: 'PHP Academy', url: 'https://phpacademy.example.com'),
	hasCourseInstance: [
		new CourseInstance(
			courseMode: 'online',
			instructor: new Person(name: 'Prof. Alice Dev'),
			courseSchedule: new Schedule(repeatFrequency: 'P1W', repeatCount: 12),
		),
	],
	offers: [
		new Offer(
			url: 'https://phpacademy.example.com/advanced-php',
			priceCurrency: 'USD',
			price: 199.0,
			itemCondition: OfferItemCondition::NewCondition,
			availability: ItemAvailability::InStock,
		),
	],
);

$json = JsonLdGenerator::SchemaToJson(schema: $course);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Course",
    "name": "Advanced PHP Development",
    "description": "Master PHP 8.x features, design patterns, and best practices.",
    "provider": {
        "@type": "Organization",
        "name": "PHP Academy",
        "url": "https://phpacademy.example.com"
    },
    "offers": [
        {
            "@type": "Offer",
            "url": "https://phpacademy.example.com/advanced-php",
            "priceCurrency": "USD",
            "price": 199,
            "itemCondition": "https://schema.org/NewCondition",
            "availability": "https://schema.org/InStock"
        }
    ],
    "hasCourseInstance": [
        {
            "@type": "CourseInstance",
            "courseMode": "online",
            "instructor": {
                "@type": "Person",
                "name": "Prof. Alice Dev"
            },
            "courseSchedule": {
                "@type": "Schedule",
                "repeatFrequency": "P1W",
                "repeatCount": 12
            }
        }
    ]
}
```

---

### Dataset

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DataDownload;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Dataset;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;

$dataset = new Dataset(
	name: 'PHP Developer Survey 2025',
	description: 'Annual survey of PHP developer demographics and tool usage.',
	url: 'https://example.com/datasets/php-survey-2025',
	creator: new Organization(name: 'PHP Foundation'),
	license: 'https://creativecommons.org/licenses/by/4.0/',
	keywords: ['PHP', 'developer survey', 'programming'],
	distribution: [
		new DataDownload(
			contentUrl: 'https://example.com/datasets/php-survey-2025.csv',
			encodingFormat: 'text/csv',
		),
	],
);

$json = JsonLdGenerator::SchemaToJson(schema: $dataset);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Dataset",
    "name": "PHP Developer Survey 2025",
    "description": "Annual survey of PHP developer demographics and tool usage.",
    "url": "https://example.com/datasets/php-survey-2025",
    "creator": {
        "@type": "Organization",
        "name": "PHP Foundation"
    },
    "license": "https://creativecommons.org/licenses/by/4.0/",
    "keywords": [
        "PHP",
        "developer survey",
        "programming"
    ],
    "distribution": [
        {
            "@type": "DataDownload",
            "contentUrl": "https://example.com/datasets/php-survey-2025.csv",
            "encodingFormat": "text/csv"
        }
    ]
}
```

---

### Discussion Forum

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DiscussionForumPosting;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;

$post = new DiscussionForumPosting(
	author: new Person(name: 'ForumUser42', url: 'https://forum.example.com/users/42'),
	datePublished: '2026-01-10T14:30:00+00:00',
	text: 'What is the best way to handle database migrations in PHP?',
	headline: 'Best approach for DB migrations?',
	url: 'https://forum.example.com/threads/db-migrations',
);

$json = JsonLdGenerator::SchemaToJson(schema: $post);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "DiscussionForumPosting",
    "author": {
        "@type": "Person",
        "name": "ForumUser42",
        "url": "https://forum.example.com/users/42"
    },
    "datePublished": "2026-01-10T14:30:00+00:00",
    "text": "What is the best way to handle database migrations in PHP?",
    "headline": "Best approach for DB migrations?",
    "url": "https://forum.example.com/threads/db-migrations"
}
```

---

### Education Q&A

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AlignmentObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Answer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Question;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Quiz;

$quiz = new Quiz(
	hasPart: [
		new Question(
			name: 'What does PHP stand for?',
			eduQuestionType: 'MultipleChoice',
			acceptedAnswer: new Answer(text: 'PHP: Hypertext Preprocessor'),
			suggestedAnswer: [
				new Answer(text: 'Personal Home Page'),
				new Answer(text: 'Hypertext Preprocessor'),
			],
		),
	],
	name: 'PHP Basics Quiz',
	about: 'PHP programming language fundamentals',
	educationalAlignment: new AlignmentObject(
		alignmentType: 'educationalLevel',
		targetName: 'Beginner',
	),
);

$json = JsonLdGenerator::SchemaToJson(schema: $quiz);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Quiz",
    "hasPart": [
        {
            "@type": "Question",
            "name": "What does PHP stand for?",
            "acceptedAnswer": {
                "@type": "Answer",
                "text": "PHP: Hypertext Preprocessor"
            },
            "suggestedAnswer": [
                {
                    "@type": "Answer",
                    "text": "Personal Home Page"
                },
                {
                    "@type": "Answer",
                    "text": "Hypertext Preprocessor"
                }
            ],
            "eduQuestionType": "MultipleChoice"
        }
    ],
    "about": "PHP programming language fundamentals",
    "educationalAlignment": {
        "@type": "AlignmentObject",
        "alignmentType": "educationalLevel",
        "targetName": "Beginner"
    },
    "name": "PHP Basics Quiz"
}
```

---

### Employer Aggregate Rating

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\EmployerAggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;

$employerRating = new EmployerAggregateRating(
	itemReviewed: new Organization(name: 'TechCorp Inc.', url: 'https://techcorp.example.com'),
	ratingValue: 4.2,
	ratingCount: 350,
	bestRating: 5,
	worstRating: 1,
);

$json = JsonLdGenerator::SchemaToJson(schema: $employerRating);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "EmployerAggregateRating",
    "itemReviewed": {
        "@type": "Organization",
        "name": "TechCorp Inc.",
        "url": "https://techcorp.example.com"
    },
    "ratingValue": 4.2,
    "ratingCount": 350,
    "bestRating": 5,
    "worstRating": 1
}
```

---

### Event

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Event;
use EvaLok\SchemaOrgJsonLd\v1\Schema\EventStatusType;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Place;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;

$event = new Event(
	name: 'PHP Conference 2026',
	startDate: '2026-06-15T09:00:00+00:00',
	location: new Place(
		name: 'Convention Center',
		address: new PostalAddress(
			streetAddress: '123 Main St',
			addressLocality: 'San Francisco',
			addressRegion: 'CA',
			postalCode: '94102',
			addressCountry: 'US',
		),
	),
	description: 'The premier PHP conference on the west coast.',
	endDate: '2026-06-17T17:00:00+00:00',
	eventStatus: EventStatusType::EventScheduled,
	image: ['https://example.com/phpconf2026.jpg'],
	organizer: new Organization(name: 'PHP Foundation', url: 'https://php.foundation'),
	offers: [
		new Offer(
			url: 'https://phpconf2026.example.com/tickets',
			priceCurrency: 'USD',
			price: 299.0,
			itemCondition: OfferItemCondition::NewCondition,
			availability: ItemAvailability::InStock,
		),
	],
);

$json = JsonLdGenerator::SchemaToJson(schema: $event);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Event",
    "name": "PHP Conference 2026",
    "startDate": "2026-06-15T09:00:00+00:00",
    "location": {
        "@type": "Place",
        "name": "Convention Center",
        "address": {
            "@type": "PostalAddress",
            "streetAddress": "123 Main St",
            "addressLocality": "San Francisco",
            "addressRegion": "CA",
            "postalCode": "94102",
            "addressCountry": "US"
        }
    },
    "description": "The premier PHP conference on the west coast.",
    "endDate": "2026-06-17T17:00:00+00:00",
    "eventStatus": "https://schema.org/EventScheduled",
    "image": [
        "https://example.com/phpconf2026.jpg"
    ],
    "offers": [
        {
            "@type": "Offer",
            "url": "https://phpconf2026.example.com/tickets",
            "priceCurrency": "USD",
            "price": 299,
            "itemCondition": "https://schema.org/NewCondition",
            "availability": "https://schema.org/InStock"
        }
    ],
    "organizer": {
        "@type": "Organization",
        "name": "PHP Foundation",
        "url": "https://php.foundation"
    }
}
```

---

### FAQ

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Answer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\FAQPage;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Question;

$faq = new FAQPage(
	mainEntity: [
		new Question(
			name: 'What is schema.org JSON-LD?',
			acceptedAnswer: new Answer(
				text: 'Schema.org JSON-LD is a method of encoding structured data in a JSON format to help search engines understand your content.',
			),
		),
		new Question(
			name: 'Does Google support JSON-LD?',
			acceptedAnswer: new Answer(
				text: 'Yes, Google recommends JSON-LD as the preferred format for structured data markup.',
			),
		),
	],
);

$json = JsonLdGenerator::SchemaToJson(schema: $faq);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "FAQPage",
    "mainEntity": [
        {
            "@type": "Question",
            "name": "What is schema.org JSON-LD?",
            "acceptedAnswer": {
                "@type": "Answer",
                "text": "Schema.org JSON-LD is a method of encoding structured data in a JSON format to help search engines understand your content."
            }
        },
        {
            "@type": "Question",
            "name": "Does Google support JSON-LD?",
            "acceptedAnswer": {
                "@type": "Answer",
                "text": "Yes, Google recommends JSON-LD as the preferred format for structured data markup."
            }
        }
    ]
}
```

---

### Image Metadata

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ImageObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;

$image = new ImageObject(
	contentUrl: 'https://example.com/photos/sunset.jpg',
	license: 'https://creativecommons.org/licenses/by/4.0/',
	acquireLicensePage: 'https://example.com/license',
	creditText: 'Example Photographer',
	creator: new Organization(name: 'Example Studio'),
	copyrightNotice: '© 2026 Example Studio',
);

$json = JsonLdGenerator::SchemaToJson(schema: $image);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "ImageObject",
    "contentUrl": "https://example.com/photos/sunset.jpg",
    "license": "https://creativecommons.org/licenses/by/4.0/",
    "acquireLicensePage": "https://example.com/license",
    "creditText": "Example Photographer",
    "copyrightNotice": "© 2026 Example Studio",
    "creator": {
        "@type": "Organization",
        "name": "Example Studio"
    }
}
```

---

### Job Posting

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\JobPosting;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MonetaryAmount;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Place;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;

$job = new JobPosting(
	title: 'Senior PHP Developer',
	description: 'We are looking for an experienced PHP developer to join our team.',
	datePosted: '2026-01-15',
	hiringOrganization: new Organization(
		name: 'TechCorp Inc.',
		url: 'https://techcorp.example.com',
		logo: 'https://techcorp.example.com/logo.png',
	),
	jobLocation: new Place(
		name: 'TechCorp HQ',
		address: new PostalAddress(
			streetAddress: '456 Tech Ave',
			addressLocality: 'Austin',
			addressRegion: 'TX',
			postalCode: '78701',
			addressCountry: 'US',
		),
	),
	baseSalary: new MonetaryAmount(currency: 'USD', minValue: 120000, maxValue: 160000),
	employmentType: 'FULL_TIME',
	validThrough: '2026-03-15',
);

$json = JsonLdGenerator::SchemaToJson(schema: $job);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "JobPosting",
    "title": "Senior PHP Developer",
    "description": "We are looking for an experienced PHP developer to join our team.",
    "datePosted": "2026-01-15",
    "hiringOrganization": {
        "@type": "Organization",
        "name": "TechCorp Inc.",
        "url": "https://techcorp.example.com",
        "logo": "https://techcorp.example.com/logo.png"
    },
    "jobLocation": {
        "@type": "Place",
        "name": "TechCorp HQ",
        "address": {
            "@type": "PostalAddress",
            "streetAddress": "456 Tech Ave",
            "addressLocality": "Austin",
            "addressRegion": "TX",
            "postalCode": "78701",
            "addressCountry": "US"
        }
    },
    "baseSalary": {
        "@type": "MonetaryAmount",
        "currency": "USD",
        "minValue": 120000,
        "maxValue": 160000
    },
    "employmentType": "FULL_TIME",
    "validThrough": "2026-03-15"
}
```

---

### Local Business

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DayOfWeek;
use EvaLok\SchemaOrgJsonLd\v1\Schema\GeoCoordinates;
use EvaLok\SchemaOrgJsonLd\v1\Schema\LocalBusiness;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OpeningHoursSpecification;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;

$localBusiness = new LocalBusiness(
	name: 'The Corner Café',
	address: new PostalAddress(
		streetAddress: '789 Oak Street',
		addressLocality: 'Portland',
		addressRegion: 'OR',
		postalCode: '97201',
		addressCountry: 'US',
	),
	url: 'https://cornercafe.example.com',
	telephone: '+15035550100',
	priceRange: '$$',
	geo: new GeoCoordinates(latitude: 45.5231, longitude: -122.6765),
	openingHoursSpecification: [
		new OpeningHoursSpecification(dayOfWeek: DayOfWeek::Monday,   opens: '07:00', closes: '18:00'),
		new OpeningHoursSpecification(dayOfWeek: DayOfWeek::Saturday, opens: '08:00', closes: '16:00'),
	],
	aggregateRating: new AggregateRating(ratingValue: 4.7, reviewCount: 218, bestRating: 5),
);

$json = JsonLdGenerator::SchemaToJson(schema: $localBusiness);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "LocalBusiness",
    "name": "The Corner Café",
    "address": {
        "@type": "PostalAddress",
        "streetAddress": "789 Oak Street",
        "addressLocality": "Portland",
        "addressRegion": "OR",
        "postalCode": "97201",
        "addressCountry": "US"
    },
    "url": "https://cornercafe.example.com",
    "telephone": "+15035550100",
    "priceRange": "$$",
    "geo": {
        "@type": "GeoCoordinates",
        "latitude": 45.5231,
        "longitude": -122.6765
    },
    "openingHoursSpecification": [
        {
            "@type": "OpeningHoursSpecification",
            "dayOfWeek": "https://schema.org/Monday",
            "opens": "07:00",
            "closes": "18:00"
        },
        {
            "@type": "OpeningHoursSpecification",
            "dayOfWeek": "https://schema.org/Saturday",
            "opens": "08:00",
            "closes": "16:00"
        }
    ],
    "aggregateRating": {
        "@type": "AggregateRating",
        "ratingValue": 4.7,
        "bestRating": 5,
        "reviewCount": 218
    }
}
```

---

### Movie

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Movie;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;

$movie = new Movie(
	name: 'The PHP Chronicles',
	image: 'https://example.com/movies/php-chronicles-poster.jpg',
	dateCreated: '2026-03-01',
	director: new Person(name: 'Alice Director'),
	aggregateRating: new AggregateRating(ratingValue: 7.5, ratingCount: 4200, bestRating: 10),
	description: 'An epic journey through the history of PHP.',
	actor: [new Person(name: 'Bob Actor'), new Person(name: 'Carol Actress')],
);

$json = JsonLdGenerator::SchemaToJson(schema: $movie);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Movie",
    "name": "The PHP Chronicles",
    "image": "https://example.com/movies/php-chronicles-poster.jpg",
    "aggregateRating": {
        "@type": "AggregateRating",
        "ratingValue": 7.5,
        "bestRating": 10,
        "ratingCount": 4200
    },
    "dateCreated": "2026-03-01",
    "director": {
        "@type": "Person",
        "name": "Alice Director"
    },
    "description": "An epic journey through the history of PHP.",
    "actor": [
        {
            "@type": "Person",
            "name": "Bob Actor"
        },
        {
            "@type": "Person",
            "name": "Carol Actress"
        }
    ]
}
```

---

### Organization

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ContactPoint;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;

$org = new Organization(
	name: 'PHP Foundation',
	url: 'https://php.foundation',
	logo: 'https://php.foundation/logo.png',
	description: 'Supporting the PHP ecosystem.',
	telephone: '+1-800-PHP-HELP',
	address: new PostalAddress(
		streetAddress: '1 Open Source Way',
		addressLocality: 'San Francisco',
		addressRegion: 'CA',
		postalCode: '94105',
		addressCountry: 'US',
	),
	contactPoint: new ContactPoint(
		telephone: '+1-800-PHP-HELP',
		contactType: 'customer support',
		availableLanguage: 'English',
	),
	sameAs: ['https://github.com/php', 'https://twitter.com/php_net'],
);

$json = JsonLdGenerator::SchemaToJson(schema: $org);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Organization",
    "name": "PHP Foundation",
    "url": "https://php.foundation",
    "logo": "https://php.foundation/logo.png",
    "description": "Supporting the PHP ecosystem.",
    "telephone": "+1-800-PHP-HELP",
    "address": {
        "@type": "PostalAddress",
        "streetAddress": "1 Open Source Way",
        "addressLocality": "San Francisco",
        "addressRegion": "CA",
        "postalCode": "94105",
        "addressCountry": "US"
    },
    "contactPoint": {
        "@type": "ContactPoint",
        "telephone": "+1-800-PHP-HELP",
        "contactType": "customer support",
        "availableLanguage": "English"
    },
    "sameAs": [
        "https://github.com/php",
        "https://twitter.com/php_net"
    ]
}
```

---

### Product

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Brand;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DefinedRegion;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MonetaryAmount;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferShippingDetails;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Product;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ShippingDeliveryTime;

$product = new Product(
	name: 'Executive Anvil',
	image: [
		'https://example.com/photos/1x1/photo.jpg',
		'https://example.com/photos/4x3/photo.jpg',
		'https://example.com/photos/16x9/photo.jpg',
	],
	description: "Sleeker than ACME's Classic Anvil, the Executive Anvil is perfect for the business traveler looking for something to drop from a height.",
	sku: '0446310786',
	brand: new Brand(name: 'ACME (tm)'),
	mpn: 'ACME0444246625',
	weight: new QuantitativeValue(value: 55.67, unitCode: 'LBR'),
	offers: [
		new Offer(
			url: 'https://example.com/anvil',
			priceCurrency: 'USD',
			price: 119.99,
			itemCondition: OfferItemCondition::NewCondition,
			availability: ItemAvailability::InStock,
			shippingDetails: [
				new OfferShippingDetails(
					shippingDestination: new DefinedRegion(addressCountry: 'US', addressRegion: ['CA', 'NV', 'AZ']),
					shippingRate: new MonetaryAmount(value: 3.49, currency: 'USD'),
					deliveryTime: new ShippingDeliveryTime(
						handlingTime: new QuantitativeValue(unitCode: 'DAY', minValue: 0, maxValue: 1),
						transitTime: new QuantitativeValue(unitCode: 'DAY', minValue: 1, maxValue: 5),
					),
				),
				new OfferShippingDetails(
					shippingDestination: new DefinedRegion(addressCountry: 'US', addressRegion: ['AK']),
					doesNotShip: true,
				),
			],
		),
	],
);

$json = JsonLdGenerator::SchemaToJson(schema: $product);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Product",
    "name": "Executive Anvil",
    "image": [
        "https://example.com/photos/1x1/photo.jpg",
        "https://example.com/photos/4x3/photo.jpg",
        "https://example.com/photos/16x9/photo.jpg"
    ],
    "description": "Sleeker than ACME's Classic Anvil, the Executive Anvil is perfect for the business traveler looking for something to drop from a height.",
    "sku": "0446310786",
    "offers": [
        {
            "@type": "Offer",
            "url": "https://example.com/anvil",
            "priceCurrency": "USD",
            "price": 119.99,
            "itemCondition": "https://schema.org/NewCondition",
            "availability": "https://schema.org/InStock",
            "shippingDetails": [
                {
                    "@type": "OfferShippingDetails",
                    "shippingDestination": {
                        "@type": "DefinedRegion",
                        "addressCountry": "US",
                        "addressRegion": ["CA", "NV", "AZ"]
                    },
                    "shippingRate": {
                        "@type": "MonetaryAmount",
                        "currency": "USD",
                        "value": 3.49
                    },
                    "deliveryTime": {
                        "@type": "ShippingDeliveryTime",
                        "handlingTime": {
                            "@type": "QuantitativeValue",
                            "unitCode": "DAY",
                            "minValue": 0,
                            "maxValue": 1
                        },
                        "transitTime": {
                            "@type": "QuantitativeValue",
                            "unitCode": "DAY",
                            "minValue": 1,
                            "maxValue": 5
                        }
                    }
                },
                {
                    "@type": "OfferShippingDetails",
                    "shippingDestination": {
                        "@type": "DefinedRegion",
                        "addressCountry": "US",
                        "addressRegion": ["AK"]
                    },
                    "doesNotShip": true
                }
            ]
        }
    ],
    "brand": {
        "@type": "Brand",
        "name": "ACME (tm)"
    },
    "mpn": "ACME0444246625",
    "weight": {
        "@type": "QuantitativeValue",
        "value": 55.67,
        "unitCode": "LBR"
    }
}
```

---

### Profile Page

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ProfilePage;

$profile = new ProfilePage(
	mainEntity: new Person(
		name: 'Alice Dev',
		url: 'https://example.com/alice',
		image: 'https://example.com/alice/photo.jpg',
		jobTitle: 'Senior PHP Developer',
		description: 'Open source contributor and PHP enthusiast.',
		sameAs: ['https://github.com/alicedev', 'https://linkedin.com/in/alicedev'],
	),
	dateCreated: '2020-01-01',
	dateModified: '2026-01-15',
);

$json = JsonLdGenerator::SchemaToJson(schema: $profile);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "ProfilePage",
    "mainEntity": {
        "@type": "Person",
        "name": "Alice Dev",
        "url": "https://example.com/alice",
        "image": "https://example.com/alice/photo.jpg",
        "jobTitle": "Senior PHP Developer",
        "sameAs": [
            "https://github.com/alicedev",
            "https://linkedin.com/in/alicedev"
        ],
        "description": "Open source contributor and PHP enthusiast."
    },
    "dateCreated": "2020-01-01",
    "dateModified": "2026-01-15"
}
```

---

### Q&A

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Answer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QAPage;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Question;

$qa = new QAPage(
	mainEntity: new Question(
		name: 'How do I install Composer?',
		text: 'I am new to PHP and want to know how to install Composer on Ubuntu.',
		answerCount: 2,
		acceptedAnswer: new Answer(
			text: 'Run: curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer',
			upvoteCount: 142,
		),
		suggestedAnswer: [
			new Answer(
				text: 'You can also download the installer from https://getcomposer.org/download/ and run it manually.',
				upvoteCount: 38,
			),
		],
	),
);

$json = JsonLdGenerator::SchemaToJson(schema: $qa);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "QAPage",
    "mainEntity": {
        "@type": "Question",
        "name": "How do I install Composer?",
        "acceptedAnswer": {
            "@type": "Answer",
            "text": "Run: curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer",
            "upvoteCount": 142
        },
        "suggestedAnswer": [
            {
                "@type": "Answer",
                "text": "You can also download the installer from https://getcomposer.org/download/ and run it manually.",
                "upvoteCount": 38
            }
        ],
        "answerCount": 2,
        "text": "I am new to PHP and want to know how to install Composer on Ubuntu."
    }
}
```

---

### Recipe

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\HowToStep;
use EvaLok\SchemaOrgJsonLd\v1\Schema\NutritionInformation;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Recipe;

$recipe = new Recipe(
	name: "Grandma's Apple Pie",
	image: [
		'https://example.com/photos/applepie-1x1.jpg',
		'https://example.com/photos/applepie-4x3.jpg',
	],
	author: new Person(name: 'Grandma Rose'),
	datePublished: '2026-01-05',
	description: 'The best apple pie recipe, passed down through generations.',
	prepTime: 'PT30M',
	cookTime: 'PT1H',
	totalTime: 'PT1H30M',
	recipeYield: '8 servings',
	recipeCategory: 'Dessert',
	recipeCuisine: 'American',
	recipeIngredient: [
		'6 large apples, peeled and sliced',
		'3/4 cup sugar',
		'1 teaspoon cinnamon',
		'2 cups all-purpose flour',
	],
	recipeInstructions: [
		new HowToStep(name: 'Prepare filling', text: 'Mix sliced apples with sugar and cinnamon in a large bowl.'),
		new HowToStep(name: 'Make crust',      text: 'Combine flour, butter, and water to form a dough. Roll out for crust.'),
		new HowToStep(name: 'Bake',            text: 'Pour filling into crust-lined pie dish, cover with top crust, bake at 375°F for 1 hour.'),
	],
	nutrition: new NutritionInformation(calories: '320 calories', fatContent: '12 g', carbohydrateContent: '52 g'),
	aggregateRating: new AggregateRating(ratingValue: 4.9, ratingCount: 1200, bestRating: 5),
);

$json = JsonLdGenerator::SchemaToJson(schema: $recipe);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Recipe",
    "name": "Grandma's Apple Pie",
    "image": [
        "https://example.com/photos/applepie-1x1.jpg",
        "https://example.com/photos/applepie-4x3.jpg"
    ],
    "author": {
        "@type": "Person",
        "name": "Grandma Rose"
    },
    "datePublished": "2026-01-05",
    "description": "The best apple pie recipe, passed down through generations.",
    "prepTime": "PT30M",
    "cookTime": "PT1H",
    "totalTime": "PT1H30M",
    "recipeYield": "8 servings",
    "recipeCategory": "Dessert",
    "recipeCuisine": "American",
    "recipeIngredient": [
        "6 large apples, peeled and sliced",
        "3/4 cup sugar",
        "1 teaspoon cinnamon",
        "2 cups all-purpose flour"
    ],
    "recipeInstructions": [
        {
            "@type": "HowToStep",
            "text": "Mix sliced apples with sugar and cinnamon in a large bowl.",
            "name": "Prepare filling"
        },
        {
            "@type": "HowToStep",
            "text": "Combine flour, butter, and water to form a dough. Roll out for crust.",
            "name": "Make crust"
        },
        {
            "@type": "HowToStep",
            "text": "Pour filling into crust-lined pie dish, cover with top crust, bake at 375°F for 1 hour.",
            "name": "Bake"
        }
    ],
    "nutrition": {
        "@type": "NutritionInformation",
        "calories": "320 calories",
        "fatContent": "12 g",
        "carbohydrateContent": "52 g"
    },
    "aggregateRating": {
        "@type": "AggregateRating",
        "ratingValue": 4.9,
        "bestRating": 5,
        "ratingCount": 1200
    }
}
```

---

### Review Snippet

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;

// Standalone review
$review = new Review(
	author: 'Jane Reviewer',
	reviewRating: new Rating(ratingValue: 4, bestRating: 5),
	reviewBody: 'Excellent product! Works exactly as described and arrived quickly.',
	datePublished: '2026-01-20',
	name: 'Great product',
);

$json = JsonLdGenerator::SchemaToJson(schema: $review);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Review",
    "author": "Jane Reviewer",
    "reviewRating": {
        "@type": "Rating",
        "ratingValue": 4,
        "bestRating": 5
    },
    "reviewBody": "Excellent product! Works exactly as described and arrived quickly.",
    "datePublished": "2026-01-20",
    "name": "Great product"
}
```

`Review` and `AggregateRating` are typically embedded inside a `Product`, `Movie`, `Recipe`, etc. rather than used standalone. The `review` and `aggregateRating` properties are available on all relevant schema types.

---

### Software App

Supports `SoftwareApplication`, `MobileApplication`, and `WebApplication`. The subtype classes share the same constructor as `SoftwareApplication`.

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MobileApplication;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\Schema\SoftwareApplication;

$app = new SoftwareApplication(
	name: 'CodeHelper Pro',
	offers: new Offer(
		url: 'https://codehelper.example.com/buy',
		priceCurrency: 'USD',
		price: 9.99,
		itemCondition: OfferItemCondition::NewCondition,
		availability: ItemAvailability::InStock,
	),
	aggregateRating: new AggregateRating(ratingValue: 4.6, ratingCount: 8900, bestRating: 5),
	applicationCategory: 'DeveloperApplication',
	operatingSystem: 'Windows, macOS, Linux',
);

$json = JsonLdGenerator::SchemaToJson(schema: $app);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "SoftwareApplication",
    "name": "CodeHelper Pro",
    "offers": {
        "@type": "Offer",
        "url": "https://codehelper.example.com/buy",
        "priceCurrency": "USD",
        "price": 9.99,
        "itemCondition": "https://schema.org/NewCondition",
        "availability": "https://schema.org/InStock"
    },
    "aggregateRating": {
        "@type": "AggregateRating",
        "ratingValue": 4.6,
        "bestRating": 5,
        "ratingCount": 8900
    },
    "applicationCategory": "DeveloperApplication",
    "operatingSystem": "Windows, macOS, Linux"
}
```

Use `MobileApplication` or `WebApplication` by substituting the class name:

```php
$mobileApp = new MobileApplication(name: 'CodeHelper Mobile', offers: $offer, aggregateRating: $rating, operatingSystem: 'ANDROID, IOS');
$webApp    = new WebApplication(name: 'CodeHelper Web',       offers: $offer, aggregateRating: $rating);
```

---

### Speakable

The `SpeakableSpecification` is used via the `speakable` property on `Article` (and its subtypes).

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Article;
use EvaLok\SchemaOrgJsonLd\v1\Schema\SpeakableSpecification;

$article = new Article(
	headline: 'Latest News: PHP Reaches New Heights',
	datePublished: '2026-01-25',
	speakable: new SpeakableSpecification(
		cssSelector: ['.headline', '.summary'],
	),
);

$json = JsonLdGenerator::SchemaToJson(schema: $article);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Article",
    "headline": "Latest News: PHP Reaches New Heights",
    "datePublished": "2026-01-25",
    "speakable": {
        "@type": "SpeakableSpecification",
        "cssSelector": [
            ".headline",
            ".summary"
        ]
    }
}
```

---

### Subscription / Paywalled Content

Mark paywalled content using `WebPageElement` via the `hasPart` property on `Article`.

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Article;
use EvaLok\SchemaOrgJsonLd\v1\Schema\WebPageElement;

$article = new Article(
	headline: 'Premium: Advanced PHP Patterns Deep Dive',
	datePublished: '2026-02-01',
	isAccessibleForFree: false,
	hasPart: new WebPageElement(
		isAccessibleForFree: false,
		cssSelector: '.premium-content',
	),
);

$json = JsonLdGenerator::SchemaToJson(schema: $article);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "Article",
    "headline": "Premium: Advanced PHP Patterns Deep Dive",
    "datePublished": "2026-02-01",
    "isAccessibleForFree": false,
    "hasPart": {
        "@type": "WebPageElement",
        "isAccessibleForFree": false,
        "cssSelector": ".premium-content"
    }
}
```

---

### Vacation Rental

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Accommodation;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BedDetails;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\VacationRental;

$rental = new VacationRental(
	name: 'Beachside Cottage',
	identifier: 'cottage-42',
	image: [
		'https://example.com/rentals/cottage-1.jpg',
		'https://example.com/rentals/cottage-2.jpg',
	],
	latitude: 36.8516,
	longitude: -75.9779,
	containsPlace: new Accommodation(
		occupancy: new QuantitativeValue(value: 4, unitCode: 'C62'),
		numberOfBedrooms: 2,
		numberOfBathroomsTotal: 1,
		bed: [
			new BedDetails(numberOfBeds: 1, typeOfBed: 'King'),
			new BedDetails(numberOfBeds: 2, typeOfBed: 'Twin'),
		],
	),
	address: new PostalAddress(
		streetAddress: '1 Ocean Drive',
		addressLocality: 'Virginia Beach',
		addressRegion: 'VA',
		postalCode: '23451',
		addressCountry: 'US',
	),
	aggregateRating: new AggregateRating(ratingValue: 4.8, reviewCount: 95, bestRating: 5),
	checkinTime: '15:00',
	checkoutTime: '11:00',
	description: 'A cozy cottage steps from the beach, perfect for a family getaway.',
);

$json = JsonLdGenerator::SchemaToJson(schema: $rental);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "VacationRental",
    "name": "Beachside Cottage",
    "identifier": "cottage-42",
    "image": [
        "https://example.com/rentals/cottage-1.jpg",
        "https://example.com/rentals/cottage-2.jpg"
    ],
    "latitude": 36.8516,
    "longitude": -75.9779,
    "containsPlace": {
        "@type": "Accommodation",
        "occupancy": {
            "@type": "QuantitativeValue",
            "value": 4,
            "unitCode": "C62"
        },
        "numberOfBedrooms": 2,
        "numberOfBathroomsTotal": 1,
        "bed": [
            {
                "@type": "BedDetails",
                "numberOfBeds": 1,
                "typeOfBed": "King"
            },
            {
                "@type": "BedDetails",
                "numberOfBeds": 2,
                "typeOfBed": "Twin"
            }
        ]
    },
    "address": {
        "@type": "PostalAddress",
        "streetAddress": "1 Ocean Drive",
        "addressLocality": "Virginia Beach",
        "addressRegion": "VA",
        "postalCode": "23451",
        "addressCountry": "US"
    },
    "aggregateRating": {
        "@type": "AggregateRating",
        "ratingValue": 4.8,
        "bestRating": 5,
        "reviewCount": 95
    },
    "checkinTime": "15:00",
    "checkoutTime": "11:00",
    "description": "A cozy cottage steps from the beach, perfect for a family getaway."
}
```

---

### Video

```php
<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\VideoObject;

$video = new VideoObject(
	name: 'Getting Started with PHP 8.3',
	thumbnailUrl: [
		'https://example.com/thumbnails/php83-tutorial-1x1.jpg',
		'https://example.com/thumbnails/php83-tutorial-16x9.jpg',
	],
	uploadDate: '2026-01-10',
	description: 'A comprehensive introduction to the new features in PHP 8.3.',
	contentUrl: 'https://example.com/videos/php83-tutorial.mp4',
	embedUrl: 'https://example.com/embed/php83-tutorial',
	duration: 'PT22M30S',
);

$json = JsonLdGenerator::SchemaToJson(schema: $video);
```

```json
{
    "@context": "https://schema.org/",
    "@type": "VideoObject",
    "name": "Getting Started with PHP 8.3",
    "thumbnailUrl": [
        "https://example.com/thumbnails/php83-tutorial-1x1.jpg",
        "https://example.com/thumbnails/php83-tutorial-16x9.jpg"
    ],
    "uploadDate": "2026-01-10",
    "description": "A comprehensive introduction to the new features in PHP 8.3.",
    "contentUrl": "https://example.com/videos/php83-tutorial.mp4",
    "embedUrl": "https://example.com/embed/php83-tutorial",
    "duration": "PT22M30S"
}
```

---

## API Reference

### `JsonLdGenerator::SchemaToJson(TypedSchema $schema): string`

Serializes any schema object to a JSON-LD string ready to embed in HTML.

```php
$json = JsonLdGenerator::SchemaToJson(schema: $mySchema);
// Embed as: <script type="application/ld+json"><?= $json ?></script>
```

**Behaviour:**
- Automatically adds `@context` (`https://schema.org/`) and `@type` (from the class constant).
- Skips `null` properties — only set properties are included in the output.
- Recursively serializes nested `TypedSchema` instances.
- Backed string enums (e.g. `ItemAvailability`, `OfferItemCondition`, `EventStatusType`) are automatically serialized to their `.value` (the full schema.org URL).
- Arrays of schema objects and primitives are both handled correctly.

### `TypedSchema` (abstract base class)

All schema classes extend `TypedSchema`. The only requirement is that each class defines the class constant:

```php
public const A_SCHEMA_TYPE = 'TypeName'; // e.g. 'Product', 'Article'
```

Schema classes use constructor promotion — all data is passed via the constructor and stored as public properties. No methods are needed beyond the constructor.

---

## Testing

```bash
# Run unit tests
composer run test-unit

# Fix code style
composer run cs-fix

# Check code style without modifying files
composer run cs-check
```

See the [test/unit](https://github.com/EvaLok/schema-org-json-ld/tree/master/test/unit) directory for the full test suite.

---

## Contributing

Contributions are welcome! Please:

1. Fork the repository and create a feature branch.
2. Add or update schema classes in `src/v1/Schema/`.
3. Write tests in `test/unit/` — see existing tests for the pattern.
4. Run `composer run cs-fix` to fix code style.
5. Run `composer run test-unit` to ensure all tests pass.
6. Open a pull request with a clear description of the change.

When adding a new schema type, follow the pattern in existing classes: extend `TypedSchema`, set `A_SCHEMA_TYPE`, use constructor promotion, and keep `null|Type` syntax for optional properties.

---

## License

MIT — see [LICENSE](LICENSE).
