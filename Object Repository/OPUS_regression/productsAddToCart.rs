<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>productsAddToCart</name>
   <tag></tag>
   <elementGuidId>e6dfaeba-00bb-48d8-a4f2-f75918c2c2ab</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;operationName\&quot;: \&quot;AddItem\&quot;,\n    \&quot;variables\&quot;: {\n        \&quot;command\&quot;: {\n            \&quot;storeId\&quot;: \&quot;opus\&quot;,\n            \&quot;userId\&quot;: \&quot;${userId}\&quot;,\n            \&quot;cultureName\&quot;: \&quot;en-US\&quot;,\n            \&quot;currencyCode\&quot;: \&quot;USD\&quot;,\n            \&quot;productId\&quot;: \&quot;${productId}\&quot;,\n            \&quot;quantity\&quot;: 1\n        }\n    },\n    \&quot;query\&quot;: \&quot;mutation AddItem($command: InputAddItemType!) {\\n  addItem(command: $command) {\\n    ...shortCartFields\\n  }\\n}\\n\\nfragment shortCartFields on OpusCartType {\\n  id\\n  itemsQuantity\\n  items {\\n    id\\n    sku\\n    quantity\\n    productId\\n    extendedPrice {\\n      amount\\n    }\\n  }\\n}\\n\&quot;\n}&quot;,
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
      <name>Cookie</name>
      <type>Main</type>
      <value>${GlobalVariable.authorizationCookie}</value>
      <webElementGuid>776c4bc5-779d-4f0d-beb8-c96a5e92805e</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/xapi/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'ca8ba2ca-1f4b-4a19-81c4-36a3591e2fc5'</defaultValue>
      <description></description>
      <id>144f89c3-ccc1-4f82-9426-7e558d031858</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>id of the selected product</description>
      <id>e3e9c7b2-31da-411c-9c4a-677eb17da266</id>
      <masked>false</masked>
      <name>productId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9ca185ac-47a1-421c-8585-7c56e74cb8ee</id>
      <masked>false</masked>
      <name>Cookie</name>
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
