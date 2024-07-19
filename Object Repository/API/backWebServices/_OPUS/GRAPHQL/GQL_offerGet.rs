<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GQL_offerGet</name>
   <tag></tag>
   <elementGuidId>33371153-ffee-496c-a655-e92d5c1b6450</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;operationName\&quot;: \&quot;GetProductOffers\&quot;,\n  \&quot;variables\&quot;: {\n    \&quot;storeId\&quot;: \&quot;${storeId}\&quot;,\n    \&quot;userId\&quot;: \&quot;${userId}\&quot;,\n    \&quot;cultureName\&quot;: \&quot;en-US\&quot;,\n    \&quot;currencyCode\&quot;: \&quot;USD\&quot;,\n    \&quot;id\&quot;: \&quot;${productId}\&quot;\n  },\n  \&quot;query\&quot;: \&quot;query GetProductOffers($storeId: String!, $currencyCode: String!, $cultureName: String, $id: String!, $userId: String!) {\\n  product(\\n    storeId: $storeId\\n    id: $id\\n    currencyCode: $currencyCode\\n    cultureName: $cultureName\\n    userId: $userId\\n  ) {\\n    id\\n    availabilityData {\\n      ...availabilityDataFields\\n    }\\n    price {\\n      actual {\\n        amount\\n        formattedAmount\\n        formattedAmountWithoutCurrency\\n        currency {\\n          symbol\\n        }\\n      }\\n      discountAmount {\\n        amount\\n        formattedAmount\\n      }\\n      sale {\\n        amount\\n        formattedAmount\\n      }\\n      list {\\n        amount\\n        formattedAmount\\n        formattedAmountWithoutCurrency\\n        currency {\\n          symbol\\n        }\\n      }\\n      discountPercent\\n    }\\n    properties(names: [\\\&quot;ContractNumber\\\&quot;]) {\\n      name\\n      value\\n    }\\n  }\\n}\\n\\nfragment availabilityDataFields on AvailabilityData {\\n  isActive\\n  isAvailable\\n  isBuyable\\n  isInStock\\n  availableQuantity\\n  isEstimated\\n}\&quot;\n}\n&quot;,
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
      <webElementGuid>c0141233-e5b2-4c87-ad29-33a320387433</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6IkQ4OEZGRUFEMTk1OUMwREUyQUE4OTI2NEFBNEQyRTQ5N0I5MTgxMEEiLCJ4NXQiOiIySV8tclJsWndONHFxSkprcWswdVNYdVJnUW8iLCJ0eXAiOiJhdCtqd3QifQ.eyJuYW1lIjoiMDgyZjQzMzQtYjQxNC00Y2JjLThlNTEtNGI4NWU3NzFkNDA1Iiwic3ViIjoiYWdlbmN5VXNlciIsImh0dHA6Ly9zY2hlbWFzLnhtbHNvYXAub3JnL3dzLzIwMDUvMDUvaWRlbnRpdHkvY2xhaW1zL2VtYWlsYWRkcmVzcyI6IjU4QGVtYWlsLmNvbSIsInJvbGUiOiJfX2N1c3RvbWVyIiwibWVtYmVySWQiOiJlMTJhMThiOC02N2RhLTRhOWItOWFlMC1jOTBhM2JkYWZlODgiLCJvaV9hdV9pZCI6Ijk5NTk2ZjJhLWQ2NWUtNDE3OS04OTI5LWUzMjFjNjlmMmY5MCIsIm9pX3Rrbl9pZCI6IjZiN2EwZGQ5LTZkN2YtNDQ0My1hOTM0LTA3ZTkzYjc5M2FmYyIsImF1ZCI6InJlc291cmNlX3NlcnZlciIsInNjb3BlIjoib2ZmbGluZV9hY2Nlc3MiLCJleHAiOjE2ODkwODEzOTQsImlzcyI6Imh0dHBzOi8vb21uaWEtYXQucGFhcy5nb3ZpcnRvLmNvbS8iLCJpYXQiOjE2ODkwNzk1OTR9.ad9J-PhY1xVNqF3VmW1rErmDNsLrr_8JWBQ0Zy_mjUpmSWKrfeDcgoi1oGpw2Teg_7nMr2DPzt0csNRezmPUul9e06GYmVfVIerbZpcTv-323DLj8rwiLCTDf7vVa5fsA3ppaEE52pbR3OiuHfJI0nGZrXz909Vib6sPyy0VmciJblc-c8Xui1kHRENqG6QJT1XSJnWcXC1MVTT4pMSShb-2yUJ1V9h6m4m-S6WWcVMuoZpy-3rbysfmGUjJZS9_qaGi4jKpCJi27fcpu0syUIDvOe7cchel6U32wIB2vufPOxVPrW2pYxHhy_VMgzMnsBiUdV0U7_0PRx03mZ3fvHfe2SIllwl6F-T3LfehtoNp94ox0oM-XOi7y3RKBr2Z9iWkVoV1Mduls-zjksvVGwIdksLozmm36TdvTIdAQ1S133XJzzsROx821kmZtmdSePTjL1OBkIhwh2gJ3q7Jnu9FvGhgsjug7u6r8ASrtxOHdOGMHdgoVCljw4PfD3NcA0KvkpzQxQQAcoAWC4MRKI0diJ2uXetb1u2OsRRYxKva8JUjJT0dZqjxqPOb5C0KdPYc4BzewsXvEzNxQe2g-AI1f72MZhaFhWRCCeEqW_FVv4nl7WXaY73MfXy5g9vaYYTbTMQGQzdLAPAYtFZ6QQ_lGSoS2DQQ4b6DcVQYpvA</value>
      <webElementGuid>776c4bc5-779d-4f0d-beb8-c96a5e92805e</webElementGuid>
   </httpHeaderProperties>
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
      <id>877818db-82bb-4530-836b-27ece9574709</id>
      <masked>false</masked>
      <name>storeId</name>
   </variables>
   <variables>
      <defaultValue>'082f4334-b414-4cbc-8e51-4b85e771d405'</defaultValue>
      <description></description>
      <id>144f89c3-ccc1-4f82-9426-7e558d031858</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>'01_100903'</defaultValue>
      <description></description>
      <id>36e065b6-d46f-4ab2-8e68-1e226a81fb01</id>
      <masked>false</masked>
      <name>productId</name>
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
