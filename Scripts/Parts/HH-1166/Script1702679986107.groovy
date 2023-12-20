import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.waitForElementPresent(findTestObject('Parts/Add Record/button_Add Record'), 10)

WebUI.click(findTestObject('Parts/Add Record/button_Add Record'))

WebUI.setText(findTestObject('Object Repository/Parts/Add Record/input_Part_Number'), 'xxxxxxxxxxxxxxxxxxxxxxxx')

WebUI.click(findTestObject('Parts/Add Record/button_Cancel Changes'))

WebUI.verifyElementNotPresent(findTestObject('Parts/Add Record/button_Save Changes'), 5)

WebUI.click(findTestObject('Parts/Add Record/button_Add Record'))

WebUI.setText(findTestObject('Object Repository/Parts/Add Record/input_Part_Name'), 'xxxxxxxxxxxxxxxxxxxxxxxx')

WebUI.click(findTestObject('Parts/Add Record/button_Cancel Changes'))

WebUI.verifyElementNotPresent(findTestObject('Parts/Add Record/button_Save Changes'), 5)

WebUI.click(findTestObject('Parts/Add Record/button_Add Record'))

WebUI.delay(2)

WebUI.click(findTestObject('Parts/Add Record/span_Select Part Type'))

WebUI.click(findTestObject('Parts/Part Types/li_SCIENCELABORATORY EQUIPMENT'))

WebUI.click(findTestObject('Parts/Add Record/button_Cancel Changes'))

WebUI.verifyElementNotPresent(findTestObject('Parts/Add Record/button_Save Changes'), 5)

WebUI.click(findTestObject('Parts/Add Record/button_Add Record'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Parts/Add Record/span_Go to last page'))

WebUI.click(findTestObject('Parts/Part Areas/li_Technology Department'))

WebUI.click(findTestObject('Parts/Add Record/button_Cancel Changes'))

WebUI.verifyElementNotPresent(findTestObject('Parts/Add Record/button_Save Changes'), 5)

WebUI.click(findTestObject('Parts/Add Record/button_Add Record'))

WebUI.setText(findTestObject('Object Repository/Parts/Add Record/input_Cost'), '5555')

WebUI.click(findTestObject('Parts/Add Record/button_Cancel Changes'))

WebUI.verifyElementNotPresent(findTestObject('Parts/Add Record/button_Save Changes'), 5)

WebUI.click(findTestObject('Parts/Add Record/button_Add Record'))

WebUI.setText(findTestObject('Object Repository/Parts/Add Record/input_MinimumStock'), '888')

WebUI.click(findTestObject('Parts/Add Record/button_Cancel Changes'))

WebUI.verifyElementNotPresent(findTestObject('Parts/Add Record/button_Save Changes'), 5)

WebUI.click(findTestObject('Parts/Add Record/button_Add Record'))

WebUI.verifyElementPresent(findTestObject('Parts/Add Record/button_Save Changes'), 5)

WebUI.click(findTestObject('Parts/Add Record/button_Cancel Changes'))

WebUI.verifyElementNotPresent(findTestObject('Parts/Add Record/button_Save Changes'), 5)

WebUI.delay(4)

