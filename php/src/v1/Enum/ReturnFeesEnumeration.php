<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Enum;

enum ReturnFeesEnumeration: string {
	case FreeReturn = 'https://schema.org/FreeReturn';
	case ReturnFeesCustomerResponsibility = 'https://schema.org/ReturnFeesCustomerResponsibility';
	case ReturnShippingFees = 'https://schema.org/ReturnShippingFees';
}
