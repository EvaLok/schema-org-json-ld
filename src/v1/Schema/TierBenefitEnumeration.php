<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum TierBenefitEnumeration: string {
	case TierBenefitLoyaltyPoints = 'https://schema.org/TierBenefitLoyaltyPoints';
	case TierBenefitLoyaltyPrice = 'https://schema.org/TierBenefitLoyaltyPrice';
}
