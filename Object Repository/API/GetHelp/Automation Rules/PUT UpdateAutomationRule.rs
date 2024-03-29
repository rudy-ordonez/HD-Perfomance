<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT UpdateAutomationRule</name>
   <tag></tag>
   <elementGuidId>ab310c4b-75b9-455b-987e-f9cb6c29a043</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;ruleID\&quot;: 0,\n\t\&quot;ruleName\&quot;: \&quot;\&quot;,\n\t\&quot;ruleDescription\&quot;: \&quot;\&quot;,\n\t\&quot;sequence\&quot;: 0,\n\t\&quot;enabled\&quot;: true,\n\t\&quot;ruleTypeID\&quot;: 1,\n\t\&quot;technicianID\&quot;: 0, \n\t\&quot;requireAll\&quot;: false,\n\t\&quot;serviceGroups\&quot;: [\n\t\t{\n\t\t\t\&quot;serviceGroupID\&quot;: 0,\n\t\t\t\&quot;serviceGroupName\&quot;: \&quot;\&quot;\n\t\t}\n\t],\n\t\&quot;ruleConditions\&quot;: [\n\t\t{\n\t\t\t\&quot;ruleConditionID\&quot;: 1,\n\t\t\t\&quot;comparisonTypeID\&quot;: 1,\n\t\t\t\&quot;comparisonValue\&quot;: \&quot;\&quot;,\n\t\t\t\&quot;fieldID\&quot;: 0,\n\t\t\t\&quot;fieldOrigin\&quot;: \&quot;System | Tag\&quot;,\n\t\t\t\&quot;order\&quot;: 1,\n\t\t\t\&quot;comparisonValueListSelections\&quot;: [ \n\t\t\t\t\&quot;\&quot; \n\t\t\t]\n\t\t}\n\t],\n\t\&quot;updatedByUserID\&quot;: 0\n}&quot;,
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
      <webElementGuid>b1eecfff-f936-46e4-9e2c-b0eb676b6ee1</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d951921c-e549-405e-8a72-95c34d3920fa</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${HHD_API_Url}/api/AutomationRules/UpdateAutomationRule</restUrl>
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
      <id>7fe0df0d-a212-4f0b-9e66-3ad0578c6d49</id>
      <masked>false</masked>
      <name>HHD_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Bearer</defaultValue>
      <description></description>
      <id>a33c5ad3-43dd-4735-8824-7e216f4de0bf</id>
      <masked>false</masked>
      <name>HHD_API_Bearer</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
