<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\Enum\DayOfWeek;
use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ServicePeriod extends TypedSchema {
	public const A_SCHEMA_TYPE = 'ServicePeriod';

	public function __construct(
		public null|QuantitativeValue $duration = null,
		/** @var DayOfWeek[] $businessDays */
		public null|array $businessDays = null,
		public null|string $cutoffTime = null,
	) {}
}
