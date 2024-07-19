<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>_debug</name>
   <tag></tag>
   <elementGuidId>3836bd18-e6a0-4f5d-8e56-34b5e0ddaf0c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n\t{\n\t\t\&quot;operationName\&quot;: \&quot;AddItem\&quot;,\n\t\t\&quot;variables\&quot;: {\n\t\t\t\&quot;command\&quot;: {\n\t\t\t\t\&quot;storeId\&quot;: \&quot;${storeId}\&quot;,\n\t\t\t\t\&quot;userId\&quot;: \&quot;${userId}\&quot;,\n\t\t\t\t\&quot;cultureName\&quot;: \&quot;${cultureName}\&quot;,\n\t\t\t\t\&quot;currencyCode\&quot;: \&quot;${currencyCode}\&quot;,\n\t\t\t\t\&quot;productId\&quot;: \&quot;${productId}\&quot;,\n\t\t\t\t\&quot;quantity\&quot;: 1\n\t\t\t}\n\t\t},\n\t\t\&quot;query\&quot;: \&quot;mutation AddItem($command: InputAddItemType!) {\\n  addItem(command: $command) {\\n    ...shortCartFields\\n  }\\n}\\n\\nfragment shortCartFields on CartType {\\n  id\\n  itemsQuantity\\n  items {\\n    id\\n    sku\\n    quantity\\n    productId\\n    extendedPrice {\\n      amount\\n    }\\n  }\\n}\\n\&quot;\n\t}\n]&quot;,
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
      <webElementGuid>87510f10-d82d-4872-a821-c3b09f8cc993</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6IkM4Rjk0QzVFQURFQUYzQjJFMDgwQTI4MkRFQjdEQUU4MTQ0OEY5RTUiLCJ4NXQiOiJ5UGxNWHEzcTg3TGdnS0tDM3JmYTZCUkktZVUiLCJ0eXAiOiJhdCtqd3QifQ.eyJuYW1lIjoiMWViMmZhOGFjNjU3NDU0MWFmZGI1MjU4MzNkYWRiNDYiLCJzdWIiOiJhZG1pbiIsImh0dHA6Ly9zY2hlbWFzLnhtbHNvYXAub3JnL3dzLzIwMDUvMDUvaWRlbnRpdHkvY2xhaW1zL2VtYWlsYWRkcmVzcyI6ImFkbWluQHZjLWRlbW9zdG9yZS5jb20iLCJyb2xlIjoiX19hZG1pbmlzdHJhdG9yIiwibWVtYmVySWQiOiIiLCJvaV9hdV9pZCI6ImY2NzFmYTJkLTAyYzgtNDI3Ni1hMTRiLThhZmI3OTI1NDYzNSIsIm9pX3Rrbl9pZCI6ImFiMjEzYjVjLWEzYzYtNGY3Ny1hYjY4LTMwNDYwZjZhODIwZCIsImF1ZCI6InJlc291cmNlX3NlcnZlciIsInNjb3BlIjoib2ZmbGluZV9hY2Nlc3MiLCJleHAiOjE2ODg2MzU2NTksImlzcyI6Imh0dHBzOi8vb21uaWEtYXQucGFhcy5nb3ZpcnRvLmNvbS8iLCJpYXQiOjE2ODg2MzM4NTl9.WgIUAzzdUYh9q6p4gdgrEKEVEpAGt_T_Q7ggKS15lOPBsTCqNp8Te1mRYBo19gUuZG8B3ySfGkZn7fPU_y75NFEEq-u58dejx00-XVufFabdVxhPPaymd-NOUqMnun7cHP42ynUN8HntPq2O0cxTxhNXuJwRCpiwJk7NuUv8-2LIUrJmN-3L7D86kkBdRTTv70VGpPelCTF5IhJ1B9Fimy00-gQolcdNuikgHJj5RilP8dyXWyAmDwCuxu8Nap6w-qlM6tqs25zl5qRKENEvjjPqzPodT5OqeGJZP5baBE7kfyR4ZIll_I8bqv2trcUCW7YwDeajTxBmmEu5vrM5Eu5xgNLL6hTAdf66QPBycM0gQfWTe_EWIGlIJuGZeaNNyZj8Coq7eKaYAkJv2XAmVjxX35YzSR_TYoIr72_Sti6W3hdvPtvBSOdF_JsSxJephTp3aTIOa5J1z5sVQ9QwvOu0OlUz80vTKmBHgNbSaQeSi5EsbnES1U6G4V7iDKA5_kW469ZwQEHedQ8gdSXz0eEooK6TvMM94fQWAIRPBcHdk_ErPQYw79pZFt1F8cq_HhZP9yiRAxlePTlzcgBkvE795rcqwXvPV2gtwuQH84BQwI8YQfwQG4hSjr7GCFQwFHOglJawR9OIm8qxg2JoP3astlJPyUqdTT-x2f9e1o8</value>
      <webElementGuid>31d1a8f8-746c-4adf-9a93-568c6dca38dc</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://omnia-at.paas.govirto.com/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'katalonStore'</defaultValue>
      <description></description>
      <id>4d473281-afbd-4571-9c93-9cb6de6f0d7e</id>
      <masked>false</masked>
      <name>storeId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8fdbc2b5-f556-4b84-918e-e916ba7c9e9b</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>'en-US'</defaultValue>
      <description></description>
      <id>3dab7bf5-fce7-4038-b3d2-efaa37e330d8</id>
      <masked>false</masked>
      <name>cultureName</name>
   </variables>
   <variables>
      <defaultValue>'01_100983'</defaultValue>
      <description></description>
      <id>eba4a8ba-53cc-431a-a0be-41dd230ca08e</id>
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
