#[doc = "Register `BK1TMGWR1` reader"]
pub type R = crate::R<BK1TMGWR1_SPEC>;
#[doc = "Register `BK1TMGWR1` writer"]
pub type W = crate::W<BK1TMGWR1_SPEC>;
#[doc = "Field `ADDRST` reader - Address setup time"]
pub type ADDRST_R = crate::FieldReader;
#[doc = "Field `ADDRST` writer - Address setup time"]
pub type ADDRST_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRHT` reader - Address-hold time"]
pub type ADDRHT_R = crate::FieldReader;
#[doc = "Field `ADDRHT` writer - Address-hold time"]
pub type ADDRHT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTST` reader - Asynchronous data setup time"]
pub type DTST_R = crate::FieldReader;
#[doc = "Field `DTST` writer - Asynchronous data setup time"]
pub type DTST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSLAT` reader - Bus latency"]
pub type BUSLAT_R = crate::FieldReader;
#[doc = "Field `BUSLAT` writer - Bus latency"]
pub type BUSLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ASYNCM` reader - Asynchronous mode"]
pub type ASYNCM_R = crate::FieldReader;
#[doc = "Field `ASYNCM` writer - Asynchronous mode"]
pub type ASYNCM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn addrst(&self) -> ADDRST_R {
        ADDRST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold time"]
    #[inline(always)]
    pub fn addrht(&self) -> ADDRHT_R {
        ADDRHT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Asynchronous data setup time"]
    #[inline(always)]
    pub fn dtst(&self) -> DTST_R {
        DTST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&self) -> BUSLAT_R {
        BUSLAT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Asynchronous mode"]
    #[inline(always)]
    pub fn asyncm(&self) -> ASYNCM_R {
        ASYNCM_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK1TMGWR1")
            .field("asyncm", &self.asyncm())
            .field("buslat", &self.buslat())
            .field("dtst", &self.dtst())
            .field("addrht", &self.addrht())
            .field("addrst", &self.addrst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn addrst(&mut self) -> ADDRST_W<'_, BK1TMGWR1_SPEC> {
        ADDRST_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Address-hold time"]
    #[inline(always)]
    pub fn addrht(&mut self) -> ADDRHT_W<'_, BK1TMGWR1_SPEC> {
        ADDRHT_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Asynchronous data setup time"]
    #[inline(always)]
    pub fn dtst(&mut self) -> DTST_W<'_, BK1TMGWR1_SPEC> {
        DTST_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&mut self) -> BUSLAT_W<'_, BK1TMGWR1_SPEC> {
        BUSLAT_W::new(self, 16)
    }
    #[doc = "Bits 28:29 - Asynchronous mode"]
    #[inline(always)]
    pub fn asyncm(&mut self) -> ASYNCM_W<'_, BK1TMGWR1_SPEC> {
        ASYNCM_W::new(self, 28)
    }
}
#[doc = "SRAM/NOR-Flash write timing registers 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmgwr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmgwr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK1TMGWR1_SPEC;
impl crate::RegisterSpec for BK1TMGWR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk1tmgwr1::R`](R) reader structure"]
impl crate::Readable for BK1TMGWR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk1tmgwr1::W`](W) writer structure"]
impl crate::Writable for BK1TMGWR1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BK1TMGWR1 to value 0x0fff_ffff"]
impl crate::Resettable for BK1TMGWR1_SPEC {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
