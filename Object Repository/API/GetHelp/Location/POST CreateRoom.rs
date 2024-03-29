<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST CreateRoom</name>
   <tag></tag>
   <elementGuidId>d3096559-2c23-4864-b6c6-d9aace18b6e3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;roomNumber\&quot;: \&quot;\&quot;,\n\t\&quot;roomDescription\&quot;: \&quot;Testing Room\&quot;,\n\t\&quot;active\&quot;: true,\n\t\&quot;createdByUserID\&quot;: \&quot;0\&quot;,\n\t\&quot;createdDate\&quot;: \&quot;2019/08/06 11:02\&quot;,\n\t\&quot;lastModifiedByUserID\&quot;: \&quot;0\&quot;,\n\t\&quot;lastModifiedDate\&quot;: \&quot;2019/08/06 11:02\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${HHD_API_Bearer}</value>
      <webElementGuid>0e0756f9-c25c-4bf5-9f65-2b8fbefbecb1</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>4e70e40e-0839-48fe-9496-e5df09ddfa26</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${HHD_API_Url}/api/Location/CreateRoom</restUrl>
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
      <id>8ce739e0-a7d7-4acd-8a24-086acfe3e465</id>
      <masked>false</masked>
      <name>HHD_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Bearer</defaultValue>
      <description></description>
      <id>c9cca367-9020-4d3e-a1d1-67639a3fa4c7</id>
      <masked>false</masked>
      <name>HHD_API_Bearer</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
