<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Event extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Event';

	public function __construct(
		public string $name,
		public string $startDate,
		public Place $location,
		public null|string $description = null,
		public null|string $endDate = null,
		public null|EventStatusType $eventStatus = null,
		/** @var string[] $image */
		public null|array $image = null,
		/** @var Offer[] $offers */
		public null|Offer|array $offers = null,
		public null|Organization|Person $organizer = null,
		/** @var Person[] $performer */
		public null|Person|array $performer = null,
		public null|string $previousStartDate = null,
	) {}
}
