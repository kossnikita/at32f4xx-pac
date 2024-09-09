#[doc = "Register `MUXH` reader"]
pub type R = crate::R<MUXH_SPEC>;
#[doc = "Register `MUXH` writer"]
pub type W = crate::W<MUXH_SPEC>;
#[doc = "Field `MUX(8-15)` reader - GPIOx pin %s muxing"]
pub type MUX_R = crate::FieldReader;
#[doc = "Field `MUX(8-15)` writer - GPIOx pin %s muxing"]
pub type MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "GPIOx pin (8-15) muxing"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `MUX8` field.</div>"]
    #[inline(always)]
    pub fn mux(&self, n: u8) -> MUX_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MUX_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "GPIOx pin (8-15) muxing"]
    #[inline(always)]
    pub fn mux_iter(&self) -> impl Iterator<Item = MUX_R> + '_ {
        (0..8).map(move |n| MUX_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 0:3 - GPIOx pin 8 muxing"]
    #[inline(always)]
    pub fn mux8(&self) -> MUX_R {
        MUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - GPIOx pin 9 muxing"]
    #[inline(always)]
    pub fn mux9(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - GPIOx pin 10 muxing"]
    #[inline(always)]
    pub fn mux10(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - GPIOx pin 11 muxing"]
    #[inline(always)]
    pub fn mux11(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - GPIOx pin 12 muxing"]
    #[inline(always)]
    pub fn mux12(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - GPIOx pin 13 muxing"]
    #[inline(always)]
    pub fn mux13(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - GPIOx pin 14 muxing"]
    #[inline(always)]
    pub fn mux14(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - GPIOx pin 15 muxing"]
    #[inline(always)]
    pub fn mux15(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXH")
            .field("mux8", &self.mux8())
            .field("mux9", &self.mux9())
            .field("mux10", &self.mux10())
            .field("mux11", &self.mux11())
            .field("mux12", &self.mux12())
            .field("mux13", &self.mux13())
            .field("mux14", &self.mux14())
            .field("mux15", &self.mux15())
            .finish()
    }
}
impl W {
    #[doc = "GPIOx pin (8-15) muxing"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `MUX8` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self, n: u8) -> MUX_W<MUXH_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MUX_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - GPIOx pin 8 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux8(&mut self) -> MUX_W<MUXH_SPEC> {
        MUX_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - GPIOx pin 9 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux9(&mut self) -> MUX_W<MUXH_SPEC> {
        MUX_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - GPIOx pin 10 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux10(&mut self) -> MUX_W<MUXH_SPEC> {
        MUX_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - GPIOx pin 11 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux11(&mut self) -> MUX_W<MUXH_SPEC> {
        MUX_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - GPIOx pin 12 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux12(&mut self) -> MUX_W<MUXH_SPEC> {
        MUX_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - GPIOx pin 13 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux13(&mut self) -> MUX_W<MUXH_SPEC> {
        MUX_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - GPIOx pin 14 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux14(&mut self) -> MUX_W<MUXH_SPEC> {
        MUX_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - GPIOx pin 15 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux15(&mut self) -> MUX_W<MUXH_SPEC> {
        MUX_W::new(self, 28)
    }
}
#[doc = "GPIO muxing function high register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXH_SPEC;
impl crate::RegisterSpec for MUXH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxh::R`](R) reader structure"]
impl crate::Readable for MUXH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxh::W`](W) writer structure"]
impl crate::Writable for MUXH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXH to value 0"]
impl crate::Resettable for MUXH_SPEC {
    const RESET_VALUE: u32 = 0;
}
