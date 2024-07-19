<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>sellerUpdate</name>
   <tag></tag>
   <elementGuidId>528b6099-afd9-4a94-a15c-9499bf8c8952</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;sellerId\&quot;: \&quot;${sellerId}\&quot;,\n\t\&quot;commissionFeeId\&quot;: \&quot;${commissionFeeId}\&quot;,\n\t\&quot;groups\&quot;: [],\n\t\&quot;sellerDetails\&quot;: {\n\t\t\&quot;name\&quot;: \&quot;${name}\&quot;,\n\t\t\&quot;approvalPolicy\&quot;: \&quot;${approvalPolicy}\&quot;,\n      \t\&quot;registrationId\&quot;: \&quot;${registrationId}\&quot;\n\t}\n}&quot;,
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
      <webElementGuid>96c47380-2fe5-460d-ade5-bfade71588f2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.token}</value>
      <webElementGuid>01eae408-7219-4a90-9ccb-1e583494d6c6</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/api/vcmp/security/seller/update</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description>update seller id</description>
      <id>1dba6ad4-e1e3-4c68-8b32-ed087b83fb1e</id>
      <masked>false</masked>
      <name>sellerId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>related comissionFee</description>
      <id>f8ad8170-703f-4a2f-b285-f569b07387e6</id>
      <masked>false</masked>
      <name>commissionFeeId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>updated vendor name</description>
      <id>36ffb63c-e1cd-4a95-88ee-b3602bc93b9c</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>updated approval policy</description>
      <id>c146518a-b6c9-4886-8f95-3f37ca078227</id>
      <masked>false</masked>
      <name>approvalPolicy</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>id to use when creating index</description>
      <id>a6a7d699-0402-4c3e-9002-4f15c9d88d7a</id>
      <masked>false</masked>
      <name>registrationId</name>
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
WS.verifyResponseStatusCode(response, 200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
