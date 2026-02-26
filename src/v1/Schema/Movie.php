<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Movie extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Movie';

	public function __construct(
		public string $name,
		public string $image,
		public null|AggregateRating $aggregateRating = null,
		public null|string $dateCreated = null,
		public null|string $datePublished = null,
		public null|Person $director = null,
		public null|Review $review = null,
		public null|string $description = null,
		/** @var Person[] $actor */
		public null|array $actor = null,
	) {}
}
