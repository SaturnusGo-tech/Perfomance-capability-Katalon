<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GQL_cartAddShipmentMethod</name>
   <tag></tag>
   <elementGuidId>9592aa77-38cf-443c-b1d0-fe4ca873c4d0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;operationName\&quot;: \&quot;AddOrUpdateCartShipment\&quot;,\n  \&quot;variables\&quot;: {\n    \&quot;command\&quot;: {\n      \&quot;storeId\&quot;: \&quot;${storeId}\&quot;,\n      \&quot;userId\&quot;: \&quot;${userId}\&quot;,\n      \&quot;cultureName\&quot;: \&quot;en-US\&quot;,\n      \&quot;currencyCode\&quot;: \&quot;USD\&quot;,\n      \&quot;cartId\&quot;: \&quot;${cartId}\&quot;,\n      \&quot;shipment\&quot;: {\n        \&quot;id\&quot;: \&quot;${shipmentId}\&quot;,\n        \&quot;price\&quot;: 10,\n        \&quot;shipmentMethodCode\&quot;: \&quot;${shipmentMethodCode}\&quot;,\n        \&quot;shipmentMethodOption\&quot;: \&quot;${shipmentMethodOption}\&quot;,\n        \&quot;vendorId\&quot;: \&quot;${vendorId}\&quot;\n      }\n    }\n  },\n  \&quot;query\&quot;: \&quot;mutation AddOrUpdateCartShipment($command: InputAddOrUpdateCartShipmentType!) {\\n  addOrUpdateCartShipment(command: $command) {\\n    ...fullCartFields\\n  }\\n}\\n\\nfragment fullCartFields on OpusCartType {\\n  id\\n  itemsQuantity\\n  purchaseOrderNumber\\n  comment\\n  availableGifts {\\n    id\\n    imageUrl\\n    name\\n    lineItemId\\n  }\\n  availableShippingMethods {\\n    id\\n    code\\n    name\\n    logoUrl\\n    optionName\\n    optionDescription\\n    price {\\n      ...moneyFields\\n    }\\n    vendorId\\n  }\\n  availablePaymentMethods {\\n    code\\n    name\\n    description\\n    logoUrl\\n    paymentMethodGroupType\\n  }\\n  items {\\n    ...lineItemFields\\n  }\\n  gifts {\\n    id\\n    imageUrl\\n    name\\n    lineItemId\\n    quantity\\n  }\\n  coupons {\\n    code\\n    isAppliedSuccessfully\\n  }\\n  discounts {\\n    description\\n    amount\\n    coupon\\n  }\\n  shipments {\\n    id\\n    shipmentMethodCode\\n    shipmentMethodOption\\n    deliveryAddress {\\n      ...cartAddressFields\\n    }\\n    price {\\n      ...moneyFields\\n    }\\n    vendor {\\n      id\\n    }\\n    comment\\n  }\\n  payments {\\n    id\\n    paymentGatewayCode\\n    billingAddress {\\n      ...cartAddressFields\\n    }\\n    vendor {\\n      id\\n    }\\n    amount {\\n      amount\\n    }\\n    comment\\n    generalLedgerNumber\\n    purchaseOrderNumber\\n    requisitionNumber\\n    dynamicProperties {\\n      name\\n    }\\n  }\\n  currency {\\n    ...currencyFields\\n  }\\n  total {\\n    ...moneyFields\\n  }\\n  discountTotal {\\n    ...moneyFields\\n  }\\n  subTotal {\\n    ...moneyFields\\n  }\\n  shippingTotal {\\n    ...moneyFields\\n  }\\n  taxTotal {\\n    ...moneyFields\\n  }\\n  vendors {\\n    vendor {\\n      id\\n      supplier {\\n        paymentMethods\\n        isOrderAllowed\\n        outerId\\n      }\\n    }\\n    subTotal {\\n      amount\\n    }\\n    shippingTotal {\\n      amount\\n    }\\n    taxTotal {\\n      amount\\n    }\\n  }\\n  warnings {\\n    ...validationErrorFields\\n  }\\n  validationErrors(ruleSet: \\\&quot;*\\\&quot;) {\\n    ...validationErrorFields\\n  }\\n  approvalFlowType\\n}\\n\\nfragment cartAddressFields on CartAddressType {\\n  id\\n  name\\n  organization\\n  firstName\\n  lastName\\n  line1\\n  line2\\n  city\\n  countryCode\\n  countryName\\n  regionId\\n  regionName\\n  postalCode\\n  phone\\n  email\\n  addressType\\n  description\\n}\\n\\nfragment lineItemFields on LineItemType {\\n  id\\n  name\\n  sku\\n  quantity\\n  inStockQuantity\\n  imageUrl\\n  selectedForCheckout\\n  productId\\n  productType\\n  product {\\n    id\\n    slug\\n    minQuantity\\n    maxQuantity\\n    masterVariation {\\n      id\\n      slug\\n    }\\n    properties {\\n      ...propertyFields\\n    }\\n    keyProperties {\\n      ...propertyFields\\n    }\\n    availabilityData {\\n      isEstimated\\n    }\\n  }\\n  vendor {\\n    id\\n    name\\n    rating {\\n      value\\n      reviewCount\\n    }\\n    supplier {\\n      paymentMethods\\n      contractNumber\\n      isOrderAllowed\\n    }\\n  }\\n  extendedPrice {\\n    ...moneyFields\\n  }\\n  placedPrice {\\n    ...moneyFields\\n  }\\n  listPrice {\\n    ...moneyFields\\n  }\\n  salePrice {\\n    ...moneyFields\\n  }\\n  discountTotal {\\n    ...moneyFields\\n  }\\n  validationErrors {\\n    errorCode\\n    errorMessage\\n    errorParameters {\\n      key\\n      value\\n    }\\n  }\\n}\\n\\nfragment propertyFields on Property {\\n  name\\n  value\\n  type\\n  hidden\\n  valueType\\n  label\\n}\\n\\nfragment moneyFields on MoneyType {\\n  amount\\n  formattedAmount\\n  formattedAmountWithoutCurrency\\n  currency {\\n    ...currencyFields\\n  }\\n}\\n\\nfragment currencyFields on CurrencyType {\\n  code\\n  symbol\\n}\\n\\nfragment validationErrorFields on ValidationErrorType {\\n  errorCode\\n  errorMessage\\n  errorParameters {\\n    key\\n    value\\n  }\\n  objectId\\n  objectType\\n}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>e3a4cdcc-f8d7-434a-8296-4a4070361064</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.token}</value>
      <webElementGuid>d4468c88-2e96-4c06-81bb-af7a35ca883e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'opus'</defaultValue>
      <description></description>
      <id>cc264b3e-f2e1-4bf0-bf67-dab7d7e91c09</id>
      <masked>false</masked>
      <name>storeId</name>
   </variables>
   <variables>
      <defaultValue>'f6a35091-d865-4c95-b70f-016a6c1e7c43'</defaultValue>
      <description></description>
      <id>e353fe4e-060c-4d9b-ba4f-d7893393d971</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>'b2350ec9-29bf-4bcb-b4e8-f17243e786de'</defaultValue>
      <description></description>
      <id>49fb4f82-407b-4921-b9f2-3bd7065adeaa</id>
      <masked>false</masked>
      <name>cartId</name>
   </variables>
   <variables>
      <defaultValue>'5d901ba5-924a-4d9f-955f-2bc765fba742'</defaultValue>
      <description></description>
      <id>fa3caa2c-ee02-4aff-afff-334f502978aa</id>
      <masked>false</masked>
      <name>shipmentId</name>
   </variables>
   <variables>
      <defaultValue>'myCoolDelivery'</defaultValue>
      <description></description>
      <id>1b4f1c0e-f44a-4e3b-8e6e-47db533d9073</id>
      <masked>false</masked>
      <name>shipmentMethodCode</name>
   </variables>
   <variables>
      <defaultValue>'c758629a-21b9-42ca-9d40-0d2a85fa91b7'</defaultValue>
      <description></description>
      <id>6aba6848-636f-4d36-a4c5-67f389fbc2cf</id>
      <masked>false</masked>
      <name>vendorId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3c4a64e6-d5f9-49ae-8036-21f7cfd87c71</id>
      <masked>false</masked>
      <name>shipmentMethodOption</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonOutput
import com.kms.katalon.core.util.KeywordUtil


ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
def prettyJson = JsonOutput.prettyPrint(response.getResponseBodyContent())
KeywordUtil.logInfo(prettyJson)
WS.verifyResponseStatusCode(response, 200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
