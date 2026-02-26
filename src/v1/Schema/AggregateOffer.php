<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class AggregateOffer extends TypedSchema {
	public const A_SCHEMA_TYPE = 'AggregateOffer';

	public function __construct(
		public float $lowPrice,
		public string $priceCurrency,
		public null|float $highPrice = null,
		public null|int $offerCount = null,
	) {}
}
