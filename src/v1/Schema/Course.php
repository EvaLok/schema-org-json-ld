<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Course extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Course';

	public function __construct(
		public string $name,
		public string $description,
		public null|Organization $provider = null,
		/** @var Offer[] $offers */
		public null|array $offers = null,
		/** @var CourseInstance[] $hasCourseInstance */
		public null|array $hasCourseInstance = null,
		public null|string $courseCode = null,
		public null|string $inLanguage = null,
		public null|int $totalHistoricalEnrollment = null,
		public null|AggregateRating $aggregateRating = null,
		public null|string $image = null,
	) {}
}
