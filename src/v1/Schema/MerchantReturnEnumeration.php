<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum MerchantReturnEnumeration: string {
	case MerchantReturnFiniteReturnWindow = 'https://schema.org/MerchantReturnFiniteReturnWindow';
	case MerchantReturnNotPermitted = 'https://schema.org/MerchantReturnNotPermitted';
	case MerchantReturnUnlimitedWindow = 'https://schema.org/MerchantReturnUnlimitedWindow';
}
