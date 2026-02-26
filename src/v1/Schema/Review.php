<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Review extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Review';

	public function __construct(
		public string|Person|Organization $author,
		public Rating $reviewRating,
		public null|string $reviewBody = null,
		public null|string $datePublished = null,
		public null|string $name = null,
		public null|TypedSchema $itemReviewed = null,
	) {}
}
