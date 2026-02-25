<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Accommodation extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Accommodation';

	public function __construct(
		public QuantitativeValue $occupancy,
		public null|string $additionalType = null,
		public null|int $numberOfBedrooms = null,
		public null|int $numberOfBathroomsTotal = null,
		public null|int $numberOfRooms = null,
		public null|QuantitativeValue $floorSize = null,
		/** @var BedDetails[] $bed */
		public null|array $bed = null,
		/** @var LocationFeatureSpecification[] $amenityFeature */
		public null|array $amenityFeature = null,
	) {}
}
