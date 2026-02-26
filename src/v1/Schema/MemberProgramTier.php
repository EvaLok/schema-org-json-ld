<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class MemberProgramTier extends TypedSchema {
	public const A_SCHEMA_TYPE = 'MemberProgramTier';

	public function __construct(
		public string $name,
		/** @var TierBenefitEnumeration[]|TierBenefitEnumeration $hasTierBenefit */
		public TierBenefitEnumeration|array $hasTierBenefit,
		public null|string $hasTierRequirement = null,
		public null|QuantitativeValue $membershipPointsEarned = null,
		public null|string $url = null,
	) {}
}
