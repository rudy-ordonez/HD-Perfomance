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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import groovy.time.TimeCategory as TimeCategory
import groovy.time.TimeDuration as TimeDuration

f = new File('C://Temp/user_perf.csv')

for (int i = 0; i < 51; i++) {
    WebUI.openBrowser('')

    WebUI.navigateToUrl(url)

    WebUI.click(findTestObject('Help Desk/Login/button_Sign In'))

    WebUI.setText(findTestObject('Help Desk/Login/input_Username_login'), username)

    WebUI.setText(findTestObject('Help Desk/Login/input_Password_pwd'), password)

    // long a1 = System.currentTimeMillis()
    start = new Date()

    WebUI.click(findTestObject('Help Desk/Login/button_Log On'))

    WebUI.waitForElementClickable(findTestObject('Help Desk/User Dashboard/select_Show All TicketsShow Open TicketsShow Resolved TicketsShow Closed Tickets'), 
        10)

    // long a2 = System.currentTimeMillis()
    end = new Date()

    // (((f << new Date(a1).format('yyyy-MM-dd HH:mm:ss.SSS')) << ',') << ((a2 - a1) / 1000)) << '\r\n'
    TimeDuration td = TimeCategory.minus(end, start)

    (f << '\n') << td // Appends result to file

    WebUI.delay(3)

    WebUI.click(findTestObject('Help Desk/Logout/User Logout/b_RudyUser Ordonez'))

    WebUI.click(findTestObject('Help Desk/Logout/User Logout/a_Log Out'))

    WebUI.closeBrowser()
}

