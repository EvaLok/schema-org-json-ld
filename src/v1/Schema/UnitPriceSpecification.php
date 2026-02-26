<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class UnitPriceSpecification extends TypedSchema {
	public const A_SCHEMA_TYPE = 'UnitPriceSpecification';

	public function __construct(
		public float $price,
		public string $priceCurrency,
		public null|string $priceType = null,
		public null|float $membershipPointsEarned = null,
		public null|MemberProgramTier $validForMemberTier = null,
		public null|QuantitativeValue $referenceQuantity = null,
	) {}
}
