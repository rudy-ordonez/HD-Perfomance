<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST CreateCustomField</name>
   <tag></tag>
   <elementGuidId>40520927-3d25-4b82-8b10-21e0832fd227</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;customFieldTypeID\&quot;: \&quot;\&quot;,\n    \&quot;fieldName\&quot;: \&quot;\&quot;,\n    \&quot;description\&quot;: \&quot;\&quot;,\n    \&quot;required\&quot;: \&quot;\&quot;,\n    \&quot;userID\&quot;: \&quot;\&quot;,\n    \&quot;optionsList\&quot;: []\n}&quot;,
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
      <webElementGuid>db8390e5-1a88-49b9-a0b0-c644cef61ab2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>eb74f147-52b6-43ce-8e4c-4140a6689f51</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${HHD_API_Url}/api/CustomFields/CreateCustomField</restUrl>
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
      <id>ab81b515-dbd2-434d-9658-35f6a3a132a1</id>
      <masked>false</masked>
      <name>HHD_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Bearer</defaultValue>
      <description></description>
      <id>5b437982-d101-4645-aede-3e35ffead472</id>
      <masked>false</masked>
      <name>HHD_API_Bearer</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
