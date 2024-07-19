<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>suppliersUpdate</name>
   <tag></tag>
   <elementGuidId>43232a84-ce84-4c4b-8e5f-6f3963543a66</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;supplierId\&quot;: \&quot;${supplierId}\&quot;,\n\t\&quot;supplierDetails\&quot;: {\n\t\t\&quot;name\&quot;: \&quot;${supplierName}\&quot;,\n\t\t\&quot;outerId\&quot;: \&quot;${supplierOuterId}\&quot;,\n\t\t\&quot;objectType\&quot;: \&quot;VirtoCommerce.CustomerModule.Core.Model.Organization\&quot;,\n\t\t\&quot;dynamicProperties\&quot;: [],\n\t\t\&quot;isActive\&quot;: true,\n\t\t\&quot;isConnected\&quot;: false,\n\t\t\&quot;supportedIntegrationType\&quot;: \&quot;None\&quot;,\n\t\t\&quot;getBalanceVendorId\&quot;: null,\n\t\t\&quot;addresses\&quot;: [],\n\t\t\&quot;phones\&quot;: [],\n\t\t\&quot;emails\&quot;: [],\n\t\t\&quot;categories\&quot;: [],\n\t\t\&quot;paymentMethods\&quot;: [],\n\t\t\&quot;dataConnections\&quot;: [\n\t\t\t{\n\t\t\t\t\&quot;dataClientTypeName\&quot;: \&quot;SupplierApiDataClient\&quot;,\n\t\t\t\t\&quot;integrationType\&quot;: \&quot;All\&quot;,\n\t\t\t\t\&quot;json\&quot;: \&quot;${json}\&quot;\n\t\t\t}\n\t\t],\n\t\t\&quot;productProperties\&quot;: [],\n\t\t\&quot;id\&quot;: \&quot;${supplierId}\&quot;,\n\t\t\&quot;iconUrl\&quot;: null,\n\t\t\&quot;supplierId\&quot;: \&quot;${supplierId}\&quot;\n\t}\n}&quot;,
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
      <webElementGuid>9e8d7034-0b5d-412c-8346-4a45437358ea</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_key</name>
      <type>Main</type>
      <value>${GlobalVariable.api_key}</value>
      <webElementGuid>981e5072-dbdb-4720-9122-c51cfa6e2e14</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/api/opus/suppliers</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'katalonSupplier'</defaultValue>
      <description>name of an added supplier</description>
      <id>60c9ce4e-16fe-401c-82fd-e6c5f082c337</id>
      <masked>false</masked>
      <name>supplierName</name>
   </variables>
   <variables>
      <defaultValue>'6f87dc3a-4f25-4616-9e47-6aef9946180c'</defaultValue>
      <description>outer id of the requested supplier</description>
      <id>8b73b7b0-36ad-4e8a-99fd-e428b6f9188c</id>
      <masked>false</masked>
      <name>supplierOuterId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>json via data connection string</description>
      <id>a0f364d1-a83c-4b76-b0f7-0dea8b448602</id>
      <masked>false</masked>
      <name>json</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>id of the created supplier</description>
      <id>25c2573f-a0f8-44ed-9933-f22a66fa2b94</id>
      <masked>false</masked>
      <name>supplierId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>123da9ff-7775-477c-a776-05b695878cd2</id>
      <masked>false</masked>
      <name>json</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

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
