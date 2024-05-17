<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST ImportSurvey</name>
   <tag></tag>
   <elementGuidId>9b44a353-9a34-42d6-91a1-8511620cba10</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;jsonrpc\&quot;:\&quot;2.0\&quot;,\n    \&quot;method\&quot;:\&quot;import_survey\&quot;,\n    \&quot;params\&quot;:[\&quot;${LimeSurvey_token}\&quot;,\&quot;${LimeSurvey_b64}\&quot;,\&quot;lss\&quot;,\&quot;${LimeSurvey_name}\&quot;,\&quot;${LimeSurvey_id}\&quot;],\n    \&quot;id\&quot;:1\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${HHD_API_Url}/index.php/admin/remotecontrol</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Url</defaultValue>
      <description></description>
      <id>18edd1e6-0806-427f-b043-44bcf1fcd3a2</id>
      <masked>false</masked>
      <name>HHDI_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LimeSurvey_token</defaultValue>
      <description></description>
      <id>3a65b98d-054f-4161-a65a-60788dbdce26</id>
      <masked>false</masked>
      <name>LimeSurvey_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LimeSurvey_b64</defaultValue>
      <description></description>
      <id>330bd728-25fa-41c4-ad43-4fc389a6b3c7</id>
      <masked>false</masked>
      <name>LimeSurvey_b64</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LimeSurvey_name</defaultValue>
      <description></description>
      <id>45c1cb18-966a-41ad-b281-80a1f431f1f9</id>
      <masked>false</masked>
      <name>LimeSurvey_name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LimeSurvey_id</defaultValue>
      <description></description>
      <id>0625fc58-cb41-4dd6-b583-f98a6f31c08b</id>
      <masked>false</masked>
      <name>LimeSurvey_id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
